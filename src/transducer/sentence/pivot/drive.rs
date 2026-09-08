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

impl Form {
    /// A18 over this form's loops.
    pub fn rank(&self) -> Option<Rank> {
        rank_of_holonomies(self.loops.iter().map(|lp| &lp.holonomy))
    }

    /// The class vector of loop `idx` (12.6): plane coordinates from the
    /// oriented holonomy; placement and hue from the affine and gauge
    /// composed around `lp.vertices` (A4).
    pub fn class_vector(&self, idx: usize) -> Option<ClassVector> {
        let lp = self.loops.get(idx)?;
        let t = loop_transport(self, lp)?;
        ClassVector::read(
            &lp.holonomy,
            t.carriage,
            CycleType::of(&t.gauge),
        )
    }

    /// Write a certified move onto the chord face's transport and re-walk.
    /// Vertices are not written.
    pub fn apply_pivot(&mut self, cert: &PivotCertificate) {
        apply_by(&mut self.faces, &cert.by);
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
        debug_assert!(
            self.vertices.iter().enumerate().all(|(i, v)| match v {
                Vertex::Word { address, .. } => self.by_address.get(address) == Some(&(i as u16)),
                _ => true,
            }),
            "by_address holds every Word vertex"
        );
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
                    .is_some_and(|f| !f.transport.word.is_identity())
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
            debug_assert_eq!(
                Some(rank_before_gauge),
                self.rank(),
                "R1: re-gauging preserves rank"
            );
            for (idx, lp) in self.loops.iter().enumerate() {
                let Some(&order) = pending.get(&vertex_ix_key(lp.chord)) else {
                    continue;
                };
                if self
                    .faces
                    .get(lp.chord as usize)
                    .is_some_and(|f| f.transport.word.is_identity())
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
            let rows: Vec<LoopRow> = self.loops.iter().map(|lp| loop_row(&lp.holonomy)).collect();
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
                let rotor_key = self.loops[idx].holonomy.rotor().class_or_rotor_key();
                let held = horizon.get(&rotor_key).is_some();
                let order = self.loops[idx].holonomy.order();
                let cands = self.candidates_for(idx, &kappa, crystal);
                let mut discharging: Vec<PivotCertificate> = Vec::new();
                for cand in &cands {
                    if !budget.spend() {
                        break;
                    }
                    if let Some(cert) = self.trial(idx, cand, &kappa, &prior) {
                        discharging.push(cert);
                    } else {
                        eprintln!(
                            "  pivot {:02x}{:02x}… loop {idx} axis {:?} no discharge",
                            idea[0], idea[1], cand.axis
                        );
                    }
                }
                match select_discharge(&discharging) {
                    Discharge::One(cert) => {
                        self.apply_pivot(&cert);
                        // A18, T21 (iii): the applied pivot's move on the
                        // whole form is the move its certificate claims.
                        debug_assert_eq!(
                            self.rank().map(|r| Rank::moved(rank_before, r)),
                            Some(cert.rank_move),
                            "the applied pivot makes the move its certificate claims"
                        );
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

    pub(crate) fn candidates_for(
        &self,
        idx: usize,
        kappa: &ClassVector,
        crystal: &dyn Crystal,
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
                    let chart = v.class();
                    charts.insert(chart.key(), chart);
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
                Axis::Hue => {
                    let Some(t) = loop_transport(self, lp) else {
                        continue;
                    };
                    out.push(Candidate {
                        axis,
                        by: PivotBy::Hue {
                            face: chord as u16,
                            gauge: t.gauge,
                        },
                    });
                }
                _ => {
                    for (_, chart) in charts.fiber(ROOT) {
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
        let mut trial_form = self.clone();
        apply_by(&mut trial_form.faces, &cand.by);
        trial_form.loops = Form::loops_of(&trial_form.faces, trial_form.vertices.len());
        let kappa_after = trial_form.class_vector(idx)?;
        if kappa_after.occupies(cand.axis) || !kappa.agrees_except(&kappa_after, cand.axis) {
            return None;
        }
        let after_h = trial_form.loops.get(idx)?.holonomy.clone();
        let mut after_rows = rows.to_vec();
        *after_rows.get_mut(idx)? = loop_row(&after_h);
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
        let class = lp.holonomy.rotor().class_or_rotor_key();
        self.torsion.insert(class, FiledLoop { sigma, period: ell, chord: lp.chord });
        true
    }

}
