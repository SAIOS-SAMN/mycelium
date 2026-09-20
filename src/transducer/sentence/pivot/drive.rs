// A16 — THE PIVOT LOOP: the one move that opens a chart's axis, and the
// candidates it reads off the form.
//
//! Candidates are read from the axes already occupied, never enumerated. A
//! trial is applied and unapplied exactly; what it measures is the class
//! vector before and after, per lane.

use super::*;

/// The period as the filing line writes it: `ℓ`, or `∞` for the `0` that
/// spells infinite order (A19, 52.3).
fn period_written(ell: u32) -> String {
    match Order::from_u32(ell) {
        Order::Finite(p) => p.to_string(),
        Order::Infinite => "∞".to_string(),
    }
}

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
    for (k, t) in loop_transport_on(faces, vertices, lp).scoped() {
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
    for (k, t) in loop_transport_on(faces, vertices, lp).scoped() {
        // THE FALLBACK IS THE PATH'S OWN WORD, NOT THE LOOP'S (54.5, as
        // ruled 2026-09-20). It read `None => lp.holonomy.clone()`, and
        // `lp.holonomy` is `form_named_holonomy` over the loop's PROJECTED
        // vertex path — a base-space object composed without the members
        // the transport lives on. `t.word` is the composite this path
        // actually walked, member by member.
        //
        // The fallback itself stays, and stripping it was wrong: a section
        // face (21 step 2, S12.2) carries a word that is DELIBERATELY not
        // the coboundary `w_j ∘ w_i⁻¹`, so a loop over one is curved on the
        // word while its edges compose to the identity. Both carriers are
        // real and the path decides which it has — removing either deletes
        // an obstruction the engine measured.
        let w = match edges.as_ref().and_then(|e| e.get(&k)) {
            Some(e) => e.word.clone(),
            None => t.word.clone(),
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
        self.loops = Self::loops_of(&self.faces, &self.vertices);
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
            self.loops = Self::loops_of(&self.faces, &self.vertices);
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
                    // §8.1, measurement only: what the loop's EDGES read
                    // (50.3) on a loop the gauge closed on the word. The
                    // gauge conjugates face words and copies the edges, so
                    // a non-flat edge here is curvature the word never held.
                    let (mut flat, mut finite, mut infinite) = (0usize, 0usize, 0usize);
                    let mut classes = String::new();
                    if let Some(hs) = self.edge_holonomies(lp) {
                        for (_, h) in hs.scoped() {
                            if h.word.is_identity() {
                                flat += 1;
                                continue;
                            }
                            match h.word.order() {
                                Order::Finite(_) => finite += 1,
                                Order::Infinite => infinite += 1,
                            }
                            if let Some(k) = h.word.rotor().class_key() {
                                use std::fmt::Write as _;
                                let _ = write!(classes, " {:02x}{:02x}…", k[0], k[1]);
                            }
                        }
                    }
                    eprintln!(
                        "  pivot {:02x}{:02x}… loop {idx} closed by the tree gauge, edges flat {flat} finite {finite} infinite {infinite}{classes}",
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
                    Discharge::NoneDischarged => {
                        let infinite = matches!(order, Order::Infinite);
                        if infinite && !held {
                            // One standing demand per chart the loop
                            // stands on, carrying the holonomy rotor as
                            // its witness (16.16 step 6): the language's
                            // want of a generator, spoken beside the
                            // filing below and never instead of it (52.3).
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
                        // A loop of any order files under its class at its
                        // period, `0` for infinite order (52.3, A19).
                        //
                        // THE PERIOD IS THE FILER'S, NOT THIS CALLER'S. It
                        // passed `order.to_u32()` — the ROTOR'S order — and
                        // a quarter turn carried along a translation has
                        // the same rotor order as one about a fixed point
                        // while closing at neither: the screw walks. The
                        // filer solves for the fixed point exactly and
                        // answers what each member path actually closes at,
                        // and the snap now carries THAT (54.3).
                        let periods = self.file_loop(idx);
                        if !periods.is_empty() {
                            if !held {
                                for ell in &periods {
                                    snaps.push(Snap::TorsionFiled { chord: idx, ell: *ell });
                                    eprintln!(
                                        "  torsion filed {:02x}{:02x}… (σ,{}) rotor order {order:?}",
                                        idea[0],
                                        idea[1],
                                        period_written(*ell)
                                    );
                                }
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
        let key = crate::transducer::sentence::demand_key(&kind, chart);
        let held = self.demands.get(&key).copied();
        match held.and_then(|i| self.vertices.get_mut(i as usize)) {
            Some(Vertex::Demand { bodies, .. }) => {
                bodies.extend(links);
                bodies.sort_unstable();
                bodies.dedup();
            }
            _ => self.vertices.push(Vertex::Demand { kind, chart: chart.clone(), bodies: links }),
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
    /// `published` is what this form had FILED BEFORE THIS STROKE BEGAN — a
    /// parameter and not a read of `self.torsion`, because `file_loop`
    /// files by class and a second loop of one class would otherwise read
    /// its own stroke's filing as an unanswered demand and widen on it. A
    /// demand nobody has had a stroke to answer is not an unanswered demand.
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
        // THE OPENING AXIS (12.6, 16.16 step 3, 3.7; I6): a loop whose
        // class has a free part, or whose class was published and not
        // answered while the basis reaches none of its coordinates, opens a
        // chart axis seeded by its own holonomy, and the class becomes a
        // coordinate of the form the court re-derives from the loop. The
        // condition is the fibre's, `kappa.free_part()`, never another
        // candidate's absence — 12.6 offers the candidates and orders
        // nothing. The order is reduce, publish, then widen: `asked` reads
        // `published`, the filings as they stood before this stroke, keyed
        // by the member paths' own classes (law 12); a finite-order loop no
        // word reaches files first (step 6, A17) and widens once the mesh
        // has not answered. The gate and the seed are the edge composite's
        // (`loop_holonomy_on`), never the word's, which telescopes on any
        // coboundary cycle. The rank audit admits the move as it stands:
        // `LoopRow::Opened` moves `torsion −1, axes +1`.
        let asked = |h: &Word| {
            h.rotor()
                .class_key()
                .is_some_and(|k| published.get(&k).is_some())
        };
        if let Some(h) = loop_holonomy_on(&self.faces, &self.vertices, lp) {
            if !held && !h.is_identity() && (kappa.free_part() || (!reached && asked(&h))) {
                if let Some(seed) = element::versor_of_word(&h) {
                    // §8.1, measurement only: the seed is the holonomy's
                    // sensor action, one plane by construction (51.2), while
                    // the axis is keyed by the whole rotor's class. Said
                    // where the rotor turns beyond that plane.
                    let off = h
                        .rotor()
                        .off_plane_blades((
                            crate::transducer::alphabet::PLANE_ROW,
                            crate::transducer::alphabet::PLANE_COL,
                        ))
                        .len();
                    if off > 0 {
                        eprintln!("  pivot open seed loop {idx} off-plane blades {off}");
                    }
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
