// A16 — THE PIVOT LOOP: the one move that opens a chart's axis, and the
// candidates it reads off the form.
//
//! Candidates are read from the axes already occupied, never enumerated. A
//! trial is applied and unapplied exactly; what it measures is the class
//! vector before and after, per lane.

use super::*;

/// WHAT A TRIAL IS MEASURED AGAINST: the form's standing before any
/// candidate is applied — the loop rows and the rank they carry, the
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
}

/// THE CLASS VECTORS OF ONE LOOP, ONE PER MEMBER PATH, read off the faces it
/// runs over and the form's axis carrier — the two things a loop's class is a
/// function of. A trial that has written one face reads its own answer here
/// without a form to read it from.
///
/// The PLANE and OFF-PLANE coordinates come from the holonomy word and are the
/// same on every path, because a word is a vertex coordinate. The PLACEMENT and
/// DILATION residues come from the carriage and the hue cycle from the gauge,
/// both member coordinates, so they are read per path (D10, item 7). An opened
/// axis zeroes the loop on every path: the holonomy IS the transition on that
/// axis and no longer a defect (I6, 15.5, T14).
pub(crate) fn class_vectors_on(
    faces: &[FormFace],
    vertices: &[Vertex],
    axes: &crate::network::nerve::Keyed<u16>,
    lp: &Loop,
) -> crate::network::nerve::Keyed<ClassVector> {
    let mut out = crate::network::nerve::Keyed::new();
    if crate::transducer::sentence::opened_axis_on(faces, vertices, axes, lp).is_some() {
        for (k, _) in lp.carriages.scoped() {
            out.insert(&k, ClassVector::zero());
        }
        // A loop with no closed member path and an opened axis still reads as
        // zero on the path the axis was opened over, which is the loop itself.
        if out.is_empty() {
            out.insert(Vec::new(), ClassVector::zero());
        }
        return out;
    }
    // THE HOLONOMIC COORDINATE IS THE EDGE COMPOSITE'S, NOT THE WORD'S, and
    // reading it off the word made 12.6's `𝒽` identically zero on every loop
    // this engine has ever perceived. A face's word is the coboundary
    // `w_j ∘ w_i⁻¹` of its two vertices' own words, and a product of
    // coboundaries around a cycle telescopes to the identity for EVERY input
    // — `loop_transport_on` says so in its own comment ("the words are vertex
    // coordinates and telescope identically whichever bodies the path runs
    // through"). Measured over 837 pair forms: 22 140 loops, 22 140 of them
    // flat on the word and not one of finite non-identity order, while the
    // edge composites over the same loops close at finite non-identity order
    // on 93 paths. The curvature was there; the coordinate that decides was
    // reading the one place it cannot appear.
    //
    // The carriage and the gauge were already the member path's own. Only the
    // rotor was taken from the loop.
    let words = holonomy_words_on(faces, vertices, lp);
    for (k, t) in loop_transport_on(faces, lp).scoped() {
        let Some(w) = words.get(&k) else {
            continue;
        };
        if let Some(cv) = ClassVector::read(w, t.carriage.clone(), CycleType::of(&t.gauge)) {
            out.insert(&k, cv);
        }
    }
    out
}

/// THE HOLONOMY WORD EACH MEMBER PATH OF A LOOP CARRIES: the edge
/// composite's word where the path has one, the loop's own word where it
/// does not.
///
/// ONE CARRIER FOR THE CHOICE (law 12). `class_vectors_on` made it and the
/// filing decision did not: `pivot_loop` read `lp.holonomy` directly for the
/// horizon key and for the order that selects the `Discharge` branch, and
/// `file_loop` filed torsion under `lp.holonomy.rotor().class_key()`. So the
/// class a loop was JUDGED by came off the edges and the class it was FILED
/// under came off the word, and those are different objects.
///
/// A face's word is the coboundary `w_j ∘ w_i⁻¹` of its two vertices' words,
/// and a product of coboundaries around a cycle telescopes to the identity,
/// so a loop over coboundary faces alone is flat on the word for every
/// input. Measured on the fleet at e98e8a2: 6535 pair forms, 70311 loops,
/// word holonomy flat 70311 — all of them — against 284 finite non-identity
/// edge composites over the same loops. The filer was keyed to the one
/// place the curvature cannot appear, so nothing was ever filed.
///
/// The word is NOT a constant by construction and is not removable: a
/// section face (21 step 2, S12.2) carries a word that is deliberately not
/// that coboundary, and a loop over one is curved on the word while its
/// edges compose to the identity. Both carriers are real; the path decides
/// which it has.
pub(crate) fn holonomy_words_on(
    faces: &[FormFace],
    vertices: &[Vertex],
    lp: &Loop,
) -> crate::network::nerve::Keyed<Word> {
    let edges = edge_holonomies_on(faces, vertices, lp);
    let mut out = crate::network::nerve::Keyed::new();
    for (k, _) in loop_transport_on(faces, lp).scoped() {
        // A path with no edge composite carries no measured transport, so the
        // only holonomy it has is the word's — which is the reading of a loop
        // whose faces are all coboundaries, not a default standing in for one.
        let w = match edges.as_ref().and_then(|e| e.get(&k)) {
            Some(e) => e.word.clone(),
            None => lp.holonomy.clone(),
        };
        out.insert(&k, w);
    }
    out
}

/// THE ONE HOLONOMY A LOOP HAS, where every member path agrees on it: the
/// edge composite wherever a path carries one, the loop's own word where it
/// does not (R8's reading, applied to the whole `OpenAxis` pipeline).
///
/// `lp.holonomy` is the WORD, and a face's word is the coboundary
/// `w_j ∘ w_i⁻¹` of its two vertices' words. A product of coboundaries
/// around a cycle telescopes to the identity for EVERY input — `∏ (w_j ∘
/// w_i⁻¹) ≡ 1`, an exact Čech coboundary, `δw = 0` — so a loop over
/// coboundary faces alone is flat on the word whatever it actually turns by.
/// Measured at e98e8a2: 70311 of 70311 loops flat on the word against 284
/// finite non-identity edge composites over those same loops.
///
/// `file_loop` was keyed to the word and filed nothing until R8 moved it
/// here. The `OpenAxis` pipeline was keyed to the word too and refused every
/// widening for the same reason: its gate is `!lp.holonomy.is_identity()`,
/// which is false by algebra on a coboundary cycle. Measured at f3a76ec:
/// `opens 1` on 133 loops — the single one whose word did not cancel —
/// against 92 torsion classes filed off the edge composite in the same five
/// minutes. The curvature and the gate were reading different carriers.
///
/// Agreement is PROJECTIVE equality of the rotors (1.5), not equality of the
/// tapes: `q` and `q·q·q` are one vertex spelled twice, and a loop is not
/// two obstructions for having been reached two ways. `None` where the paths
/// disagree — the loop then has no single holonomy and a caller that keyed a
/// carrier by one path's answer would be asserting the rest (rule 11).
/// A LOOP CARRYING NO MEMBER PATH ANSWERS THE WORD. `holonomy_words_on`
/// already rules that a path with no edge composite carries the word — "the
/// only holonomy it has is the word's, which is the reading of a loop whose
/// faces are all coboundaries, not a default standing in for one" — and a
/// loop with no path at all has no edge composite anywhere, so the same
/// reading applies with nothing to read it over. Returning `None` there made
/// the RANK unmeasured on every such form: `rank_offplane_is_its_own_lane`
/// went from a measured `(offplane 1, free 0)` to no rank at all.
pub(crate) fn loop_holonomy_on(
    faces: &[FormFace],
    vertices: &[Vertex],
    lp: &Loop,
) -> Option<Word> {
    let ws = holonomy_words_on(faces, vertices, lp);
    let mut held: Option<Word> = None;
    for (_, w) in ws.scoped() {
        match &held {
            None => held = Some(w.clone()),
            Some(h) if !h.rotor().projective_eq(&w.rotor()) => return None,
            Some(_) => {}
        }
    }
    // Written as the two readings they are, not as a fallback: a loop with
    // member paths answers what those paths agree on, and a loop with none
    // answers its word, because a loop with no path has no edge composite to
    // answer instead (rule 11 — this is not a default for a missing value).
    match held {
        Some(w) => Some(w),
        None => Some(lp.holonomy.clone()),
    }
}

impl Form {
    /// The class vector of loop `idx` (12.6): plane coordinates from the
    /// oriented holonomy; placement and hue from the carriage and gauge
    /// composed around `lp.vertices` (A4). A loop whose class is an axis
    /// the form opened reads as the zero vector: its holonomy is the
    /// transition on that axis and no longer a defect (I6, 15.5, T14).
    /// THE TRANSPORT COMPOSITE, which is one of a loop's two obstructions
    /// and not the only one. `FormFace.edge` carries the other — 50.3's
    /// stabilizer element — and it is read by [`Form::edge_class_vector`].
    /// Measured: reading the edge HERE instead zeroed the class of the one
    /// loop in the tree the pivot actually fires on (a turned L across two
    /// pairs, occupying `Unit(0)`), because an extraction's faces carry an
    /// edge whose composite is the identity while their transports carry
    /// the half turn. They are two objects and neither replaces the other.
    /// ONE PER MEMBER PATH (D10 item 7), keyed by the path.
    pub fn class_vectors(&self, idx: usize) -> crate::network::nerve::Keyed<ClassVector> {
        match self.loops.get(idx) {
            Some(lp) => class_vectors_on(&self.faces, &self.vertices, &self.axes, lp),
            None => crate::network::nerve::Keyed::new(),
        }
    }

    /// THE LOOP'S CLASS WHERE EVERY MEMBER PATH AGREES ON IT, which is what a
    /// caller asking "is this loop a defect" is asking: a loop is a defect if
    /// ANY path of it carries an unfiled class, and the readings differ only in
    /// the placement and dilation residues.
    ///
    /// `None` when the paths disagree — the loop then has no single class and
    /// a caller that treated one path's answer as the loop's would be making
    /// exactly the assertion D10 removes — or when no path closes.
    pub fn class_vector(&self, idx: usize) -> Option<ClassVector> {
        let vs = self.class_vectors(idx);
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

    /// THE HOLONOMY WORD EVERY MEMBER PATH OF LOOP `idx` AGREES ON, by the
    /// same rule [`Form::class_vector`] collapses the class vectors: the
    /// paths' own readings, and absence where they differ.
    ///
    /// Agreement is projective equality of the rotors (1.5) and not equality
    /// of the tapes, because `q` and `q·q·q` are one vertex spelled twice and
    /// a loop is not two obstructions for having been reached two ways.
    pub fn holonomy_word(&self, idx: usize) -> Option<Word> {
        loop_holonomy_on(&self.faces, &self.vertices, self.loops.get(idx)?)
    }

    /// Write a certified move onto the form — the chord face's transport,
    /// or an axis opened over the loop — and re-walk. Vertices are not
    /// written.
    pub fn apply_pivot(&mut self, cert: &PivotCertificate) {
        self.apply_move(&cert.by);
        self.loops = Self::loops_of(&self.faces, self.vertices.len());
        self.reindex();
    }

    /// A16 on the form. Certificates and vortex live here.
    /// `bundle` is the pairs the idea germinated from; `t_extent` is how
    /// long the loop has been held on the bubble's clock.
    pub fn pivot_loop(
        &mut self,
        idea: impl AsRef<[u8]>,
        horizon: &Keyed<(Rotor, Order, Standing)>,
        crystal: &dyn Crystal,
        budget: &mut Budget,
        bundle: Option<&crate::transducer::family::Family>,
        t_extent: u64,
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
        let mut handled: Keyed<()> = Keyed::new();
        loop {
            if budget.remaining() == 0 {
                break;
            }
            self.loops = Self::loops_of(&self.faces, self.vertices.len());
            let mut pending: Keyed<Order> = Keyed::new();
            for lp in &self.loops {
                if self
                    .faces
                    .get(lp.chord as usize)
                    .is_some_and(|f| !f.word.is_identity())
                {
                    pending.insert(vertex_ix_key(lp.chord), lp.holonomy.order());
                }
            }
            let Some(rank_before_gauge) = self.rank() else {
                eprintln!(
                    "  pivot {:02x}{:02x}… held, rank overflow",
                    idea[0], idea[1]
                );
                break;
            };
            if let Some(gamma) = self.solve_coboundary() {
                if self.regauge(&gamma).is_none() {
                    eprintln!(
                        "  pivot {:02x}{:02x}… held, tree gauge cannot be applied",
                        idea[0], idea[1]
                    );
                    break;
                }
            }
            // R1: re-gauging preserves rank, certified here as a move (§2.3,
            // law 10): `Equal` continues, a discharge continues with its
            // coordinate named, and only `Inadmissible` holds the form.
            let after_gauge = self.rank();
            match after_gauge.map(|a| Rank::moved(rank_before_gauge, a)) {
                None => {
                    eprintln!("  pivot {:02x}{:02x}… held, rank overflow", idea[0], idea[1]);
                    break;
                }
                Some(RankMove::Equal) => {}
                Some(RankMove::Discharged { from }) => {
                    eprintln!(
                        "  pivot {:02x}{:02x}… the tree gauge discharged {from:?}",
                        idea[0], idea[1]
                    );
                }
                Some(RankMove::Inadmissible { delta }) => {
                    eprintln!(
                        "  pivot {:02x}{:02x}… held, the tree gauge moved the rank by {delta:?} \
                         (torsion, free, off-plane, axes)",
                        idea[0], idea[1]
                    );
                    break;
                }
            }
            for (idx, lp) in self.loops.iter().enumerate() {
                let Some(&order) = pending.get(&vertex_ix_key(lp.chord)) else {
                    continue;
                };
                if self
                    .faces
                    .get(lp.chord as usize)
                    .is_some_and(|f| f.word.is_identity())
                {
                    snaps.push(Snap::Closed {
                        chord: idx,
                        order,
                    });
                    eprintln!(
                        "  pivot {:02x}{:02x}… loop {idx} closed by the tree gauge",
                        idea[0], idea[1]
                    );
                }
            }
            let open: Vec<usize> = self
                .loops
                .iter()
                .enumerate()
                .filter(|(i, _)| self.class_vector(*i).is_some_and(|k| !k.is_zero()))
                .map(|(i, _)| i)
                .collect();
            if open.is_empty() {
                break;
            }
            // A loop whose holonomy is not a versor has no row (15.3): the
            // audit is unmeasured and the form is held, never counted
            // under a key of the rotor's bytes.
            let Some(rows) = self.loop_rows() else {
                eprintln!(
                    "  pivot {:02x}{:02x}… held, rank unmeasured",
                    idea[0], idea[1]
                );
                break;
            };
            let Some(rank_before) = rank_of_rows(&rows) else {
                eprintln!(
                    "  pivot {:02x}{:02x}… held, rank overflow",
                    idea[0], idea[1]
                );
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
            };
            let mut applied = false;
            for idx in open {
                if !budget.spend() {
                    break;
                }
                let Some(kappa) = self.class_vector(idx) else {
                    continue;
                };
                let loop_k = vertex_ix_key(idx as u16);
                if handled.contains(&loop_k) {
                    continue;
                }
                // The holonomy the member paths carry — the edge composite
                // wherever a path has one — is the class the loop is judged
                // and filed by (law 12); `open` holds only loops whose paths
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
                // The span reading, measured and never branched on: `held`
                // counts the words `crystal.on_axis` answered — the basis's
                // reach on the loop's occupied axes; `opens` the puncture,
                // offered on a free part; `self` the loop's own inverses,
                // which say nothing about what the mind holds. `unreached`
                // and `starved` say whether the fixed axis order of 12.6 let
                // the budget reach every candidate.
                let kind = |c: &&Candidate| match c.axis {
                    Axis::Unit(_) | Axis::Prime5(_) | Axis::Prime13(_) | Axis::OffPlane => 0,
                    Axis::Open => 1,
                    _ => 2,
                };
                let held_gens = cands.iter().filter(|c| kind(c) == 0).count();
                let opens = cands.iter().filter(|c| kind(c) == 1).count();
                let self_moves = cands.iter().filter(|c| kind(c) == 2).count();
                // `ruled` says whether the mesh holds the class (A19) — a
                // loop on the horizon neither demands nor widens, it waits
                // for the held form — and `occupies` names the coordinates
                // the loop stands on, so an empty candidate list is read by
                // its cause: `Hue` with no member path, or an axis the
                // crystal holds no word on.
                let occupied = kappa.coordinates();
                let mut tried = 0usize;
                let mut starved = false;
                let mut discharging: Vec<PivotCertificate> = Vec::new();
                let mut eliminated = 0usize;
                for cand in &cands {
                    // The zero-spend eliminator: whether a held word can
                    // cancel this loop's rotor is exact arithmetic in the
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
                                eliminated += 1;
                                continue;
                            }
                        }
                    }
                    if !budget.spend() {
                        starved = true;
                        break;
                    }
                    tried += 1;
                    if let Some(cert) = self.trial(idx, cand, &kappa, &prior) {
                        discharging.push(cert);
                    } else {
                        eprintln!(
                            "  pivot {:02x}{:02x}… loop {idx} axis {:?} no discharge",
                            idea[0], idea[1], cand.axis
                        );
                    }
                }
                // `tried` counts one per element of `cands` and nothing else,
                // so this is a difference of two counts of the same set and
                // not a floor at zero (rule 11).
                let unreached = cands.len() - tried;
                eprintln!(
                    "  pivot candidates {:02x}{:02x}… loop {idx} total {} held {held_gens} \
                     self {self_moves} opens {opens} tried {tried} unreached {unreached} \
                     discharged {} starved {starved} eliminated {eliminated} span {} \
                     ruled {held} order {order:?} occupies {occupied:?}",
                    idea[0],
                    idea[1],
                    cands.len(),
                    discharging.len(),
                    // The span reading in one word. `understood`: a held
                    // generator reaches an occupied axis, so the obstruction
                    // factors through the basis. `novel`: none does and the
                    // puncture is offered, so the residue is outside the span
                    // and opening a dimension is the only algebraic move
                    // left. `closed`: neither — the loop's coordinates are
                    // all self-derived and no pivot reaches it by any route.
                    match (held_gens, opens) {
                        (0, 0) => "closed",
                        (0, _) => "novel",
                        _ => "understood",
                    }
                );
                // 37.7 from the loop's own residue: the basis reached no
                // coordinate this loop occupies, the class was published and
                // nothing answered, so the rotor the loop measured is carried
                // out to be adjoined — beside the widening, never instead of
                // it. Minting is the entity's, since a form does not hold the
                // language; it stands in the dialect, `Stood` not `Ruled`, so
                // it suppresses no demand the mesh may still answer.
                if held_gens == 0
                    && !held
                    && !hw.rotor().is_identity()
                    && hw.rotor().class_key().is_some_and(|k| published.get(&k).is_some())
                {
                    // Every chart the loop stands on (rule 15): a loop through
                    // vertices on two charts is an obstruction on both.
                    let charts = self.charts_of_loop(idx);
                    for chart in &charts {
                        snaps.push(Snap::Adjoin {
                            chord: idx,
                            rotor: Box::new(hw.rotor()),
                            chart: chart.clone(),
                        });
                    }
                    if !charts.is_empty() {
                        eprintln!(
                            "  adjoin from residue {:02x}{:02x}… loop {idx} charts {}",
                            idea[0],
                            idea[1],
                            charts.len()
                        );
                    }
                }
                // One certificate per move (`one_per_move`): a word
                // discharging on both sides of the chord is one move spelled
                // twice, never a vortex of itself.
                let moves = one_per_move(&discharging);
                if moves.len() != discharging.len() {
                    eprintln!(
                        "  pivot {:02x}{:02x}… loop {idx} {} certificates spell {} moves",
                        idea[0],
                        idea[1],
                        discharging.len(),
                        moves.len()
                    );
                }
                match outcome_of(&moves, starved) {
                    Discharge::One(cert) => {
                        self.apply_pivot(&cert);
                        // A18, T21 (iii): the applied pivot makes the move
                        // its certificate claims, certified here (law 10).
                        // On disagreement the loop holds — no snap, no
                        // certificate — and nothing downstream rests on an
                        // uncertified move.
                        if self.rank().map(|r| Rank::moved(rank_before, r)) != Some(cert.rank_move) {
                            eprintln!(
                                "  pivot {:02x}{:02x}… loop {idx} held, the applied pivot did not make the move its certificate claims",
                                idea[0], idea[1]
                            );
                            break;
                        }
                        eprintln!(
                            "  pivot {:02x}{:02x}… loop {idx} axis {:?} discharged {:02x}{:02x}…→{:02x}{:02x}…",
                            idea[0], idea[1], cert.axis, cert.before[0], cert.before[1], cert.after[0], cert.after[1]
                        );
                        let ck = cert.key();
                        self.certificates.insert(ck, (*cert).clone());
                        snaps.push(Snap::Discharged {
                            chord: idx,
                            certificate: cert,
                        });
                        handled.insert(loop_k, ());
                        applied = true;
                        break;
                    }
                    Discharge::Vortex(held_certs) => {
                        for c in &held_certs {
                            vortex_put(&mut self.vortex, c.clone());
                        }
                        if prior.mu_before.is_some() {
                            let between = obstructions(&held_certs);
                            eprintln!(
                                "  vortex {:02x}{:02x}… loop {idx} {} candidates held, axes {:?}",
                                idea[0],
                                idea[1],
                                held_certs.len(),
                                between.iter().map(|o| o.axes).collect::<Vec<_>>()
                            );
                            snaps.push(Snap::Vortex {
                                chord: idx,
                                held: held_certs,
                                between,
                            });
                        } else {
                            eprintln!(
                                "  vortex {:02x}{:02x}… loop {idx} {} candidates unmeasured",
                                idea[0],
                                idea[1],
                                held_certs.len()
                            );
                            snaps.push(Snap::Unmeasured {
                                chord: idx,
                                held: held_certs,
                            });
                        }
                        handled.insert(loop_k, ());
                    }
                    // THE STROKE ENDED, NOT THE SEARCH. Nothing is spoken,
                    // filed or widened on an evaluation that did not
                    // happen: the loop is held and the next stroke reaches
                    // the candidates this one could not.
                    Discharge::Starved => {
                        eprintln!(
                            "  pivot {:02x}{:02x}… loop {idx} held, the candidate search was cut short at {tried} of {}",
                            idea[0],
                            idea[1],
                            cands.len()
                        );
                        handled.insert(loop_k, ());
                    }
                    Discharge::NoneDischarged => match order {
                        Order::Infinite => {
                            if !held {
                                // One standing demand per chart the loop
                                // stands on, carrying the holonomy rotor as
                                // its witness (16.16 step 6).
                                let charts = self.charts_of_loop(idx);
                                for chart in &charts {
                                    snaps.push(Snap::Demand {
                                        chord: idx,
                                        key: rotor_key.clone(),
                                        rotor: Box::new(hw.rotor()),
                                        chart: chart.clone(),
                                    });
                                }
                                eprintln!(
                                    "  demand spoken {:02x}{:02x}… order ∞ charts {}",
                                    idea[0],
                                    idea[1],
                                    charts.len()
                                );
                            }
                            handled.insert(loop_k, ());
                        }
                        Order::Finite(ell) => {
                            if self.file_loop(idx, ell) {
                                if !held {
                                    snaps.push(Snap::TorsionFiled { chord: idx, ell });
                                    eprintln!(
                                        "  torsion filed {:02x}{:02x}… (σ,{ell})",
                                        idea[0], idea[1]
                                    );
                                }
                            } else {
                                snaps.push(Snap::TorsionOpen { chord: idx });
                                eprintln!(
                                    "  torsion open {:02x}{:02x}… chord {idx}",
                                    idea[0], idea[1]
                                );
                            }
                            handled.insert(loop_k, ());
                        }
                    },
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
        cells: Vec<crate::transducer::sentence::Cell>,
    ) {
        let key = crate::transducer::sentence::demand_key(&kind, chart);
        let held = self.demands.get(&key).copied();
        match held.and_then(|i| self.vertices.get_mut(i as usize)) {
            Some(Vertex::Demand { bodies, .. }) => {
                bodies.extend(cells);
                bodies.sort_unstable();
                bodies.dedup();
            }
            _ => self.vertices.push(Vertex::Demand { kind, chart: chart.clone(), bodies: cells }),
        }
        self.reindex();
    }

    /// Every chart a loop stands on, once each, in key order: the charts of
    /// its vertices, never one of them by position (rule 15).
    fn charts_of_loop(&self, idx: usize) -> Vec<stalk::Invariant> {
        let mut at: Keyed<stalk::Invariant> = Keyed::new();
        if let Some(lp) = self.loops.get(idx) {
            for vi in &lp.vertices {
                let Some(v) = self.vertices.get(*vi as usize) else {
                    continue;
                };
                for chart in v.charts() {
                    at.insert(chart.key(), chart);
                }
            }
        }
        at.scoped().map(|(_, c)| c.clone()).collect()
    }

    /// The candidate pivots of a held loop (16.16 step 3), read off its
    /// class vector in the fixed order, each instantiated from what the
    /// crystal holds. `held` is the horizon's word: the mesh holds this
    /// class already (A19), so the mind neither demands it nor opens an
    /// axis on it; it is still discharged by a held form when one arrives.
    /// `published` is what this form had FILED BEFORE THIS STROKE BEGAN, and
    /// it is a parameter rather than a read of `self.torsion` because those
    /// are two different facts. `file_loop` files by CLASS, so two loops of
    /// one class are published by whichever of them the drive reaches first
    /// — and the second would then read its own stroke's filing as an
    /// unanswered demand and widen on it, microseconds after it was made.
    /// Measured: `two_loops_of_one_class_are_both_filed_or_discharged` filed
    /// one and widened the other. A demand that nobody has had a stroke to
    /// answer is not an unanswered demand.
    pub(crate) fn candidates_for(
        &self,
        idx: usize,
        kappa: &ClassVector,
        crystal: &dyn Crystal,
        held: bool,
        published: &Keyed<FiledLoop>,
    ) -> Vec<Candidate> {
        let Some(lp) = self.loops.get(idx) else {
            return Vec::new();
        };
        let chord = lp.chord as usize;
        if self.faces.get(chord).is_none() {
            return Vec::new();
        }
        let mut charts: Keyed<stalk::Invariant> = Keyed::new();
        for &fi in &lp.faces {
            let Some(f) = self.faces.get(fi as usize) else {
                continue;
            };
            for vi in [f.i, f.j] {
                if let Some(v) = self.vertices.get(vi as usize) {
                    // EVERY CHART THE VERTEX STANDS AT, so a vertex acting on
                    // two contributes both and the loop's chart set is the
                    // whole of what the loop passes through.
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
                    let Some(translator) = element::translator_of(&kappa.carriage) else {
                        continue;
                    };
                    out.push(Candidate {
                        axis,
                        by: PivotBy::Placement {
                            face: chord as u16,
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
                            face: chord as u16,
                            translator,
                        },
                    });
                }
                Axis::Orientation(_) => {
                    out.push(Candidate {
                        axis,
                        by: PivotBy::Word {
                            face: chord as u16,
                            word: crate::transducer::alphabet::PlaneForm::sheet_word(),
                            side: Side::Right,
                        },
                    });
                }
                // ONE HUE CANDIDATE PER MEMBER PATH. The gauge the move
                // carries is the loop's composite hue map, which is a member
                // coordinate, so a loop running through two pairs of bodies
                // offers the move each of them names. Offering one and
                // skipping the rest is a move never offered on the others.
                Axis::Hue => {
                    for (_, t) in loop_transport(self, lp).scoped() {
                        out.push(Candidate {
                            axis,
                            by: PivotBy::Hue {
                                face: chord as u16,
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
                    // THE CHART THE ASK IS MADE AT (§8.1, measurement only).
                    // `on_axis_of` is an EXACT keyed read — it does not peel
                    // the way `generators_under` does — so a word filed under
                    // one chart and looked up under another is invisible, and
                    // an empty answer cannot tell the two apart. `axis_admit`
                    // files under `stalk::invariant_of(element)`, the
                    // SENTENCE's own chart; this asks under the vertices'
                    // `acts` charts. Whether those coincide is not something
                    // to reason about — it is a pair of keys, and both are
                    // now on the wire.
                    for (_, chart) in charts.scoped() {
                        eprintln!(
                            "  axis ask chart {:?} axis {axis:?} answered {}",
                            chart.0,
                            crystal.on_axis(chart, axis).len()
                        );
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
                                        face: chord as u16,
                                        word: w.clone(),
                                        side,
                                    },
                                });
                            }
                        }
                    }
                }
            }
        }
        // THE OPENING AXIS (12.6, 16.16 step 3, 3.7; I6). A residual
        // measures how narrow the cover was: a loop whose class has a free
        // part, which the working atlas holds no word or form on and the
        // mesh does not hold, opens a chart axis seeded by its own holonomy.
        // This is read off the fibre — it is empty — and never off a
        // trial's failure; a finite-order loop with no word files as
        // torsion instead (step 6, A17). The class becomes a coordinate of
        // the form, and the court re-derives the seed from the loop.
        //
        // `out.is_empty() &&` STOOD HERE and is deleted. It made the opening
        // axis a last resort: the move that widens a cover was offered only
        // where no other candidate had been offered at all, so a loop whose
        // class has a free part — which is exactly the reading that says the
        // cover is too narrow — kept that reading to itself whenever any
        // word happened to be spellable on its chord. The condition is the
        // FIBRE's, and the fibre is `kappa.free_part()`; whether some other
        // candidate exists is a fact about the alphabet, not about the
        // narrowness this move answers. 12.6 is explicit that the candidates
        // are offered and nothing orders them, so a gate that suppresses one
        // candidate because another exists is an ordering by construction.
        //
        // AND IT IS OFFERED ON AN IRREDUCIBLE RESIDUAL THAT HAS ALREADY
        // BEEN PUBLISHED AND NOT ANSWERED, which is the second half of the
        // same reading. `free_part()` is ONE way a cover can be too narrow
        // — the class runs off the plane or carries a prime exponent — and
        // it is not the only one. A class occupying a plane coordinate the
        // basis holds no word on is just as far outside the span, and a
        // pivot is a REDUCTION: it divides the residual by a generator.
        // Where no generator reaches the coordinate there is nothing to
        // divide by, every contraction candidate is terminal by algebra,
        // and asking again next stroke asks the same question of the same
        // basis and gets the same answer forever. That is the trap, and the
        // move out of it is the reduction's inverse: stop dividing and
        // widen the cover, so the class is re-read as a coordinate of the
        // form rather than a defect in it.
        //
        // MEASURED, 373 loops at 769a17c: every loop with no candidate at
        // all occupies `[Unit(0)]`, 60 of 60, every crystal read answered 0,
        // 123 of 123, and not one had a free part — so this move was
        // offered zero times and every one of them filed as torsion, every
        // stroke, and was asked again.
        //
        // THE ORDER IS REDUCE, PUBLISH, THEN WIDEN, and the third step
        // waits on the second. Offered on irreducibility ALONE this
        // pre-empts 12.6 step 6 entirely: the discharging expansion wins
        // `select_discharge` and the loop never reaches the filing arm, so
        // a class the mind cannot divide is buried under a wider cover
        // instead of being published for a mind that CAN divide it to
        // answer. Measured directly — six fixtures that file a finite
        // holonomy against an empty crystal stopped filing, and the whole
        // filed-torsion path (cauterize, the relator, the court's cocycles)
        // went dead. Filing is not a failure to pivot; it is the demand,
        // and widening before making it is answering a question nobody was
        // asked.
        //
        // So `asked` is read off `published` — the state of the carrier
        // `file_loop` writes, as it stood BEFORE this stroke, keyed by the
        // member paths' own classes through `holonomy_words_on`, which is
        // the one reader the filing uses (law 12). A class filed by an
        // earlier stroke has been put to the mesh and nothing has come back;
        // THAT is the residual with no move left, and it is the one this
        // widens.
        //
        // An infinite-order residual needs none of this: it carries a prime
        // exponent or runs off the plane, so `free_part()` already holds and
        // the expansion is offered on the first stroke, beside the demand.
        //
        // THE RANK AUDIT ALREADY ADMITS THE MOVE. `loop_row_on` reads an
        // opened axis as `LoopRow::Opened`, so a torsion loop that opens
        // moves `torsion −1, axes +1` and `Rank::moved` names it
        // `Discharged { from: Torsion }`. This needs no widening of that
        // gate: `[0, 0, 0, +1]` is a DIFFERENT move — an axis opened by a
        // loop that contributed no coordinate — and nothing here makes one.
        //
        // THE GATE AND THE SEED ARE THE EDGE COMPOSITE'S, NOT THE WORD'S.
        // `!lp.holonomy.is_identity()` stood here, and `lp.holonomy` is the
        // word: a face's word is the coboundary `w_j ∘ w_i⁻¹` of its two
        // vertices' words, and `∏ (w_j ∘ w_i⁻¹) ≡ 1` around any cycle by
        // algebra. So the gate asked whether an exact Čech coboundary was
        // non-trivial, which it never is, and the widening was shut off on
        // every loop over coboundary faces — every loop this engine sees.
        // Measured at f3a76ec: `opens 1` on 133 loops, the single one whose
        // word happened not to cancel, against 92 torsion classes filed off
        // the edge composite in the same five minutes. The curvature and the
        // gate were reading different carriers.
        //
        // `loop_holonomy_on` is the composite, and the seed comes from the
        // same object: seeded from the word, the axis was seeded with the
        // identity versor and stood for nothing the loop turns by.
        let asked = |h: &Word| {
            h.rotor()
                .class_key()
                .is_some_and(|k| published.get(&k).is_some())
        };
        if let Some(h) = loop_holonomy_on(&self.faces, &self.vertices, lp) {
            if !held && !h.is_identity() && (kappa.free_part() || (!reached && asked(&h))) {
                if let Some(seed) = element::versor_of_word(&h) {
                    out.push(Candidate {
                        axis: Axis::Open,
                        by: PivotBy::OpenAxis {
                            face: chord as u16,
                            seed,
                        },
                    });
                }
            }
        }
        out
    }

}
