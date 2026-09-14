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
    pub(crate) rows: &'a [LoopRow],
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
    if crate::transducer::sentence::opened_axis_on(axes, lp).is_some() {
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
                Some(h) if h != cv => return None,
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
        let lp = self.loops.get(idx)?;
        let ws = holonomy_words_on(&self.faces, &self.vertices, lp);
        let mut held: Option<Word> = None;
        for (_, w) in ws.scoped() {
            match &held {
                None => held = Some(w.clone()),
                Some(h) if !h.rotor().projective_eq(&w.rotor()) => return None,
                Some(_) => {}
            }
        }
        held
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
        // THE MEET IS RE-DERIVED, NOT ASSERTED. `by_address` is how A6 and
        // Mayer–Vietoris find the vertex carrying an address, and a stroke
        // that reads a stale one reads the wrong word — Law 2, under an
        // index instead of a digest. This stood as a `debug_assert!` that
        // the caller had reindexed, which is nothing at all in release: the
        // form went on to be pivoted with an empty meet. Deriving it here
        // costs one pass over the vertices and faces, which `apply_pivot`
        // already pays on every applied move.
        self.reindex();
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
            // R1: RE-GAUGING PRESERVES RANK, certified here. This stood as
            // a `debug_assert_eq!`, which is nothing in a release build,
            // and a release build is what the mesh runs -- so a gauge that
            // moved the rank went on to be pivoted from, and every
            // certificate the drive then issued measured its move against
            // a rank the theorem says it should not have had. A theorem
            // asserted only in a debug build is not certified (law 10:
            // direction is a pivot, closure certifies). It HOLDS, in the
            // vocabulary the arms above it already use.
            //
            // IT IS READ AS A MOVE, NOT AS AN INEQUALITY. `self.rank() !=
            // Some(rank_before_gauge)` collapsed the four-coordinate audit
            // onto one bool and broke the whole drive on it, so a gauge that
            // DISCHARGED a coordinate — the move A18 names, and the one the
            // certificate is issued for — stopped the pivot exactly as a
            // corrupt one would, and the line said only "moved the rank".
            // §2.3 has the reading: `Rank::moved` names which coordinate a
            // unit left and carries the delta whole when nothing admissible
            // happened. R1's theorem is that re-gauging holds the rank, so
            // `Equal` continues; a discharge is admissible and continues,
            // with the coordinate named; only `Inadmissible` breaks, and it
            // breaks saying what the four coordinates actually did.
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
                rows: &rows,
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
                // THE HOLONOMY THE PATHS ACTUALLY CARRY, which is the edge
                // composite wherever a path has one. This read
                // `self.loops[idx].holonomy` — the word — for both the
                // horizon key and the order that picks the `Discharge`
                // branch, while `class_vector` two lines above already read
                // the edge composite. A loop was judged by one object and
                // filed under another.
                //
                // `open` holds only loops whose paths AGREE on a class
                // vector, and the class is a coordinate of that vector, so a
                // loop reaching here has one holonomy class across its paths
                // and `holonomy_word` answers it rather than a path taken by
                // position.
                let Some(hw) = self.holonomy_word(idx) else {
                    continue;
                };
                let Some(rotor_key) = hw.rotor().class_key() else {
                    continue;
                };
                let held = horizon.get(&rotor_key).is_some();
                let order = hw.order();
                let cands = self.candidates_for(idx, &kappa, crystal, held);
                // THE SPAN READING (§8.1, measurement only — nothing below
                // branches on it).
                //
                // WHAT IS NOT MEASURABLE HERE, stated because measuring it is
                // the obvious move and it returns a constant. "Candidates on
                // an axis `kappa` does not occupy" is not a reading of this
                // corpus or any other: `candidates_for` builds every non-Open
                // candidate by walking `kappa.coordinates()`, which yields
                // only occupied axes, and it tags the puncture `Axis::Open`,
                // which `ClassVector::occupies` answers `false` for
                // unconditionally. So that count is exactly the OpenAxis
                // count on every input, and a count that is a constant by
                // construction asserts nothing.
                //
                // WHAT THE CANDIDATES DO SEPARATE is where each one came
                // from, and that is the span question `factor_on_plane`
                // answers. `factor_on_plane` itself takes a budgeted crystal
                // read and would change the thing this line measures, but
                // `candidates_for` has ALREADY made the read:
                //
                //   - `Unit`, `Prime5`, `Prime13` and `OffPlane` candidates
                //     are the words `crystal.on_axis` returned. They are HELD
                //     GENERATORS, and one existing means the obstruction
                //     factors through the basis — `Understood`.
                //   - `Orientation`, `Placement`, `Dilation` and `Hue`
                //     candidates are built from `kappa`'s own carriage, gauge
                //     and the sheet word. They are the residue re-presented
                //     as its own inverse and hold no information about what
                //     the mind knows.
                //   - `Open` is the puncture, offered only on a free part.
                //
                // So `held` counts the basis's reach. Where it is zero the
                // residue lies strictly outside the span, every contraction
                // in the existing basis is provably terminal, and the one
                // algebraic move left is the puncture — which is exactly what
                // `Rank::moved` refuses as `[0, 0, 0, +1]`.
                //
                // THE STARVATION READING rides the same line. The loop below
                // spends a unit per candidate and BREAKS on exhaustion, so
                // candidates early in 12.6's fixed axis order can drink the
                // budget before one later in that order is ever reached.
                // `unreached` is how many were never tried; with `starved` it
                // says whether a form's closing pivot is unreachable every
                // stroke, which is a candidate mechanism for the 2495 forms
                // that never discharge.
                let kind = |c: &&Candidate| match c.axis {
                    Axis::Unit(_) | Axis::Prime5(_) | Axis::Prime13(_) | Axis::OffPlane => 0,
                    Axis::Open => 1,
                    _ => 2,
                };
                let held_gens = cands.iter().filter(|c| kind(c) == 0).count();
                let opens = cands.iter().filter(|c| kind(c) == 1).count();
                let self_moves = cands.iter().filter(|c| kind(c) == 2).count();
                // WHAT THE LOOP OCCUPIES, because the counts above cannot say
                // why a list is empty and the first reading came back empty on
                // 342 of 373 loops. A loop reaching here is non-zero — `open`
                // filtered on `!k.is_zero()` — so it occupies at least one
                // coordinate, and exactly three things turn an occupied
                // coordinate into no candidate: `Hue` where `loop_transport`
                // carries no member path, and `Unit`/`Prime5`/`Prime13`/
                // `OffPlane` where `crystal.on_axis` holds no word. The axes
                // separate those, and nothing else in this line does.
                let occupied = kappa.coordinates();
                let mut tried = 0usize;
                let mut starved = false;
                let mut discharging: Vec<PivotCertificate> = Vec::new();
                for cand in &cands {
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
                     discharged {} starved {starved} span {} occupies {occupied:?}",
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
                match select_discharge(&discharging) {
                    Discharge::One(cert) => {
                        self.apply_pivot(&cert);
                        // A18, T21 (iii): THE APPLIED PIVOT MAKES THE MOVE
                        // ITS CERTIFICATE CLAIMS, certified here. A
                        // `debug_assert_eq!` certified it in debug builds
                        // and in nothing else, so in the build the mesh
                        // runs a certificate claiming one move while the
                        // form made another was recorded, spoken, and
                        // settled on -- law 10 says closure certifies, and
                        // a closure nobody checks certifies nothing.
                        //
                        // On disagreement the loop HOLDS: no snap, no
                        // certificate filed, and `applied` stays false so
                        // the drive stops rather than pivoting again from
                        // a form it cannot account for. What is NOT done
                        // is undoing the transport already written -- the
                        // move is a legitimate one on the form, it is the
                        // CLAIM about it that fails, and nothing
                        // downstream may rest on an uncertified move
                        // because nothing downstream is given one.
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
                    Discharge::NoneDischarged => match order {
                        Order::Infinite => {
                            if !held {
                                snaps.push(Snap::Demand {
                                    chord: idx,
                                    key: rotor_key,
                                });
                                eprintln!(
                                    "  demand spoken {:02x}{:02x}… order ∞",
                                    idea[0], idea[1]
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

    /// The candidate pivots of a held loop (16.16 step 3), read off its
    /// class vector in the fixed order, each instantiated from what the
    /// crystal holds. `held` is the horizon's word: the mesh holds this
    /// class already (A19), so the mind neither demands it nor opens an
    /// axis on it; it is still discharged by a held form when one arrives.
    pub(crate) fn candidates_for(
        &self,
        idx: usize,
        kappa: &ClassVector,
        crystal: &dyn Crystal,
        held: bool,
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
                    // The charts THIS loop stands on, gathered above from
                    // its own classes: the candidate moves are read on each
                    // of them, and one skipped is a move never offered.
                    for (_, chart) in charts.scoped() {
                        for w in crystal.on_axis(chart, axis) {
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
        if !held && kappa.free_part() && !lp.holonomy.is_identity() {
            if let Some(seed) = element::versor_of_word(&lp.holonomy) {
                out.push(Candidate {
                    axis: Axis::Open,
                    by: PivotBy::OpenAxis {
                        face: chord as u16,
                        seed,
                    },
                });
            }
        }
        out
    }

    pub(crate) fn trial(
        &self,
        idx: usize,
        cand: &Candidate,
        kappa: &ClassVector,
        before: &Prior<'_>,
    ) -> Option<PivotCertificate> {
        let &Prior { rows, rank_before, crystal, bundle, t_extent, ref mu_before } = before;
        // THE TRIAL OWNS WHAT THE MOVE WRITES AND BORROWS THE REST. A
        // `Word`, `Placement` or `Hue` move writes ONE FACE's transport and
        // nothing else, so the trial owns the faces and reads the loops
        // they close into; the form's vertices, its indices and its
        // carriers are unchanged and are not copied for a candidate that is
        // about to be refused.
        //
        // Opening an axis writes the form's axis carrier as well, and that
        // carrier is small: the trial owns a copy of it for that arm.
        let mut faces = self.faces.clone();
        let mut axes = self.axes.clone();
        apply_by(&mut faces, &cand.by);
        let loops = Form::loops_of(&faces, self.vertices.len());
        // OPENING AN AXIS files the loop's class against the chord it closes
        // on (I6). A move that does not apply — no loop on that chord, no
        // class, no seed to re-derive it from — is not a candidate that
        // failed to discharge; there is no trial to read (rule 11).
        if let PivotBy::OpenAxis { face, .. } = &cand.by {
            let lp = loops.iter().find(|lp| lp.chord == *face)?;
            let class = lp.holonomy.rotor().class_key()?;
            self.axis_seed_of(lp)?;
            axes.insert(class, *face);
        }
        // 12.6: a pivot on axis x discharges when x reads zero after and no
        // other coordinate moved; opening an axis discharges the LOOP — it
        // is no longer a loop on the refined cover (15.5, T14) — so its
        // whole vector reads zero.
        //
        // EVERY MEMBER PATH MUST DISCHARGE. The trial is a write on a FACE,
        // which relates every pair of bodies the face relates, so a move that
        // zeroes the coordinate along one path and leaves it standing on
        // another has not discharged the loop — it has discharged one reading
        // of it, and the pivot certificate would assert the rest. A loop with
        // no closed member path has nothing to read and refuses (rule 11).
        // The trial rewrites the faces and reopens the axes; the VERTICES are
        // untouched by it, so the Frame is read off the form's own.
        let after = class_vectors_on(&faces, &self.vertices, &axes, loops.get(idx)?);
        if after.is_empty() {
            return None;
        }
        let discharged = after.scoped().all(|(_, kappa_after)| {
            if cand.axis == Axis::Open {
                kappa_after.is_zero()
            } else {
                !kappa_after.occupies(cand.axis) && kappa.agrees_except(kappa_after, cand.axis)
            }
        });
        if !discharged {
            return None;
        }
        // THE CERTIFICATE NAMES ONE CLASS AFTER, so it is only writable where
        // every member path agrees on it. The plane and off-plane coordinates
        // are path-independent; a residue that differs between paths means the
        // loop has no single class and there is nothing to certify (rule 11).
        let mut agreed: Option<ClassVector> = None;
        for (_, cv) in after.scoped() {
            match &agreed {
                None => agreed = Some(cv.clone()),
                Some(h) if h != cv => return None,
                Some(_) => {}
            }
        }
        let kappa_after = agreed?;
        let mut after_rows = rows.to_vec();
        *after_rows.get_mut(idx)? =
            crate::transducer::sentence::loop_row_on(&axes, loops.get(idx)?)?;
        let rank_after = rank_of_rows(&after_rows)?;
        // T21 (iii): the rank holds, or one unit moves out of torsion, free
        // or off-plane and into axes. Anything else is not this pivot's move
        // and the trial refuses it here, naming what the audit did.
        let rank_move = Rank::moved(rank_before, rank_after);
        if let RankMove::Inadmissible { delta } = rank_move {
            eprintln!(
                "  pivot loop {idx} axis {:?} refused: rank moved {delta:?}",
                cand.axis
            );
            return None;
        }
        let mu = match mu_before {
            None => Mu::Unmeasured,
            Some((pairs_before, before)) => {
                let (pairs_before, before) = (pairs_before, *before);
                // THE BUNDLE IS SPOKEN THROUGH A WHOLE FORM, so this is
                // where one is built — for a candidate that has already
                // discharged its axis and held the rank, which is the few.
                // Every candidate paid for it before, including the ones
                // refused two readings above.
                let trial_form = Form {
                    faces: faces.clone(),
                    loops: loops.clone(),
                    axes: axes.clone(),
                    ..self.clone()
                };
                let (pairs_after, after) = crate::transducer::family::bundle_pressure(
                    &trial_form,
                    bundle?,
                    crystal,
                    t_extent,
                )?;
                // 12.6: a trial refuses a candidate that raises any lane on
                // ANY pair, and the reading names the pairs. Two readings of
                // different lengths are not the same bundle and are not
                // comparable, which is also a refusal — of the comparison.
                let raised =
                    crate::transducer::family::pairs_raised(pairs_before, &pairs_after)?;
                if !raised.is_empty() {
                    return None;
                }
                // 12.6, T21 (iv): a candidate that RAISES any lane is
                // refused, read per lane and named. Strictness is not
                // asked: a pivot that moves no lane is certified by its
                // closure, not by a fall.
                let reading = crate::economy::lane_moves(&after, &before);
                let risen = crate::economy::lanes_risen(&reading);
                if !risen.is_empty() {
                    eprintln!(
                        "  pivot loop {idx} axis {:?} refused: lanes {risen:?} rose",
                        cand.axis
                    );
                    return None;
                }
                Mu::Measured { before, after }
            }
        };
        Some(PivotCertificate {
            chord: idx as u16,
            axis: cand.axis,
            before: kappa.class_key(),
            after: kappa_after.class_key(),
            rank_move,
            mu,
            by: cand.by.clone(),
        })
    }

    /// FILE THE LOOP'S TORSION UNDER EVERY CLASS ITS MEMBER PATHS CARRY.
    ///
    /// Torsion is filed under the holonomy's CLASS (16.17, 15.4), and the
    /// holonomy of a path is its edge composite where it has one. This read
    /// `lp.holonomy.rotor().class_key()` — the word — so on a corpus where
    /// every loop is flat on the word (70311 of 70311 at e98e8a2) it filed
    /// the identity's class or nothing at all, while the 284 curved edge
    /// composites over those same loops were filed nowhere.
    ///
    /// One class per member path, not one for the loop: a loop whose faces
    /// relate several pairs of bodies has one composite per path through
    /// them, and each is its own obstruction. A path that carries no versor
    /// has no class and is skipped; a loop where no path files returns
    /// `false` and stays open, which is what `torsion open` reports.
    pub(crate) fn file_loop(&mut self, idx: usize, ell: u32) -> bool {
        if ell == 0 {
            return false;
        }
        let Some(lp) = self.loops.get(idx) else {
            return false;
        };
        let sigma: Vec<(u8, u8)> = lp
            .vertices
            .windows(2)
            .map(|p| (p[0] as u8, p[1] as u8))
            .collect();
        let chord = lp.chord;
        let words = holonomy_words_on(&self.faces, &self.vertices, lp);
        let mut filed = false;
        for (_, w) in words.scoped() {
            let Some(class) = w.rotor().class_key() else {
                continue;
            };
            self.torsion.insert(
                class,
                FiledLoop { sigma: sigma.clone(), period: ell, chord },
            );
            filed = true;
        }
        filed
    }

}
