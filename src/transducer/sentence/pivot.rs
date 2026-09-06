// AI-GENERATED CONTENT — see DISCLAIMER.md.

//! Pivot loop, class vector, and rank.

use super::class::*;
use super::form::*;
use super::tape::from_element_form;
use super::transport::*;
use crate::network::nerve::{Keyed, ROOT};
use crate::physics::Budget;
use crate::transducer::alphabet::{Order, Rotor, Word};
use crate::transducer::element::Element;
use crate::transducer::family::Crystal;
use crate::transducer::stalk::{self, Standing};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Snap {
    /// A chord the tree gauge closed: its class was trivial (T2), never rank.
    Closed { chord: usize, order: Order },
    /// A held loop discharged on one occupied axis by a certified pivot.
    Discharged { chord: usize, certificate: PivotCertificate },
    /// Several pivots discharge and none dominates on μ⃗: all are held as
    /// cells of the thought until a perception, a glue or a ruling collapses
    /// the superposition (12.6).
    Vortex { chord: usize, held: Vec<PivotCertificate> },
    /// Several discharges with no bundle to measure μ⃗.
    Unmeasured { chord: usize, held: Vec<PivotCertificate> },
    TorsionFiled { chord: usize, ell: u32 },
    TorsionOpen { chord: usize },
    Demand { chord: usize, key: [u8; 32] },
}

/// The three outcomes of selecting among certified discharges.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Discharge {
    One(PivotCertificate),
    Vortex(Vec<PivotCertificate>),
    NoneDischarged,
}

/// A remainder cell's kind, read from the two hues. Hue 0 is void.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellKind {
    Created,
    Erased,
    Recoloured,
    Unchanged,
}

impl CellKind {
    pub fn of(before: u8, after: u8) -> CellKind {
        match (before == 0, after == 0) {
            (true, true) => CellKind::Unchanged,
            (true, false) => CellKind::Created,
            (false, true) => CellKind::Erased,
            (false, false) if before == after => CellKind::Unchanged,
            (false, false) => CellKind::Recoloured,
        }
    }
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
        Some(ClassVector::read(
            &lp.holonomy,
            t.shift,
            CycleType::of(&t.gauge),
        ))
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
        idea: [u8; 32],
        horizon: &Keyed<(Rotor, Order, Standing)>,
        crystal: &dyn Crystal,
        budget: &mut Budget,
        bundle: Option<&[(Vec<Vec<u8>>, Vec<Vec<u8>>)]>,
        t_extent: u64,
    ) -> Vec<Snap> {
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
                break;
            };
            match self.solve_coboundary() {
                Some(gamma) => {
                    if self.regauge(&gamma).is_none() {
                        eprintln!(
                            "  pivot {:02x}{:02x}… held, tree gauge cannot be applied",
                            idea[0], idea[1]
                        );
                        break;
                    }
                }
                None => {}
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
                break;
            };
            let mu_before: Option<[u64; 4]> = match bundle {
                None => None,
                Some(b) => crate::transducer::family::bundle_pressure(self, b, crystal, t_extent),
            };
            let mut applied = false;
            for idx in open {
                if !budget.spend() {
                    break;
                }
                let Some(kappa) = self.class_vector(idx) else {
                    continue;
                };
                let key = kappa.class_key();
                let loop_k = vertex_ix_key(idx as u16);
                if handled.contains(&loop_k) {
                    continue;
                }
                let held = horizon
                    .get(&self.loops[idx].holonomy.rotor().class_key())
                    .is_some();
                let order = self.loops[idx].holonomy.order();
                let cands = self.candidates_for(idx, &kappa, crystal);
                let mut discharging: Vec<PivotCertificate> = Vec::new();
                for cand in &cands {
                    if !budget.spend() {
                        break;
                    }
                    if let Some(cert) = self.trial(
                        idx,
                        cand,
                        &kappa,
                        &rows,
                        rank_before,
                        crystal,
                        bundle,
                        t_extent,
                        mu_before,
                    ) {
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
                        debug_assert_eq!(cert.rank_before, rank_before);
                        self.apply_pivot(&cert);
                        debug_assert!(
                            self.rank().is_some_and(|r| Rank::admits(rank_before, r)),
                            "A18: an applied pivot does not lose rank"
                        );
                        debug_assert_eq!(self.rank(), Some(cert.rank_after));
                        eprintln!(
                            "  pivot {:02x}{:02x}… loop {idx} axis {:?} discharged {:02x}{:02x}…→{:02x}{:02x}…",
                            idea[0], idea[1], cert.axis, cert.before[0], cert.before[1], cert.after[0], cert.after[1]
                        );
                        let ck = cert.key();
                        self.certificates.insert(ck, cert.clone());
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
                        if mu_before.is_some() {
                            eprintln!(
                                "  vortex {:02x}{:02x}… loop {idx} {} candidates held",
                                idea[0],
                                idea[1],
                                held_certs.len()
                            );
                            snaps.push(Snap::Vortex {
                                chord: idx,
                                held: held_certs,
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
                                snaps.push(Snap::Demand { chord: idx, key });
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

    fn candidates_for(
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
                    out.push(Candidate {
                        axis,
                        by: PivotBy::Placement {
                            face: chord as u16,
                            shift: kappa.placement.clone(),
                        },
                    });
                }
                Axis::Hue => {
                    let gauge = loop_transport(self, lp)
                        .map(|t| t.gauge)
                        .unwrap_or_default();
                    out.push(Candidate {
                        axis,
                        by: PivotBy::Hue {
                            face: chord as u16,
                            gauge,
                        },
                    });
                }
                _ => {
                    for (_, chart) in charts.fiber(&ROOT) {
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

    fn trial(
        &self,
        idx: usize,
        cand: &Candidate,
        kappa: &ClassVector,
        rows: &[LoopRow],
        rank_before: Rank,
        crystal: &dyn Crystal,
        bundle: Option<&[(Vec<Vec<u8>>, Vec<Vec<u8>>)]>,
        t_extent: u64,
        mu_before: Option<[u64; 4]>,
    ) -> Option<PivotCertificate> {
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
        if !Rank::admits(rank_before, rank_after) {
            return None;
        }
        let mu = match mu_before {
            None => Mu::Unmeasured,
            Some(before) => {
                let after = crate::transducer::family::bundle_pressure(
                    &trial_form,
                    bundle?,
                    crystal,
                    t_extent,
                )?;
                if !(after == before || crate::economy::descends(&after, &before)) {
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
            rank_before,
            rank_after,
            mu,
            by: cand.by.clone(),
        })
    }

    fn file_loop(&mut self, idx: usize, ell: u32) -> bool {
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
        let class = lp.holonomy.rotor().class_key();
        self.torsion.insert(class, (sigma, ell, lp.chord));
        true
    }
    pub fn loops_of(faces: &[FormFace], n: usize) -> Vec<Loop> {
        if n == 0 {
            return Vec::new();
        }
        let mut parent: Vec<usize> = (0..n).collect();
        let find = |parent: &mut [usize], mut x: usize| -> usize {
            while parent[x] != x {
                parent[x] = parent[parent[x]];
                x = parent[x];
            }
            x
        };
        let mut adj: Vec<Vec<(usize, usize)>> = vec![Vec::new(); n];
        let mut loops = Vec::new();
        let mut order = Vec::with_capacity(faces.len());
        for (fi, f) in faces.iter().enumerate() {
            if f.i == 0 || f.j == 0 {
                order.push(fi);
            }
        }
        for (fi, f) in faces.iter().enumerate() {
            if f.i != 0 && f.j != 0 {
                order.push(fi);
            }
        }
        for fi in order {
            let f = &faces[fi];
            let (a, b) = (f.i as usize, f.j as usize);
            if a >= n || b >= n {
                continue;
            }
            let (pa, pb) = (find(&mut parent, a), find(&mut parent, b));
            if pa != pb {
                parent[pa] = pb;
                adj[a].push((b, fi));
                adj[b].push((a, fi));
                continue;
            }
            let Some((mut path, mut walked)) = tree_path(&adj, a, b) else {
                continue;
            };
            path.push(a);
            walked.push(fi);
            let holonomy = form_named_holonomy(faces, &path, &walked);
            loops.push(Loop {
                vertices: path.into_iter().map(|x| x as u16).collect(),
                faces: walked.into_iter().map(|x| x as u16).collect(),
                chord: fi as u16,
                holonomy,
            });
        }
        loops
    }
}

pub(crate) fn loop_transport(form: &Form, lp: &Loop) -> Option<Transport> {
    let mut acc_word = Word::identity();
    let mut acc_shift = q2_zero();
    let mut acc_gauge = Vec::new();
    for (win, &fi) in lp.vertices.windows(2).zip(&lp.faces) {
        let f = form.faces.get(fi as usize)?;
        let step = if f.i == win[0] && f.j == win[1] {
            f.transport.clone()
        } else {
            f.transport.inverse()?
        };
        let mapped = apply_linear(&step.word, &acc_shift)?;
        acc_shift = (mapped.0 + &step.shift.0, mapped.1 + &step.shift.1);
        acc_word = step.word.compose(&acc_word);
        acc_gauge = compose_gauge(&step.gauge, &acc_gauge);
    }
    Some(Transport {
        word: acc_word.normal_form(),
        shift: acc_shift,
        gauge: acc_gauge,
    })
}

pub(crate) fn apply_by(faces: &mut [FormFace], by: &PivotBy) {
    match by {
        PivotBy::Word { face, word, side } => {
            if let Some(f) = faces.get_mut(*face as usize) {
                f.transport.word = pivoted_face(&f.transport.word, word, *side);
            }
        }
        PivotBy::Placement { face, shift } => {
            if let Some(f) = faces.get_mut(*face as usize) {
                f.transport.shift = (
                    &f.transport.shift.0 + &shift.0,
                    &f.transport.shift.1 + &shift.1,
                );
            }
        }
        PivotBy::Hue { face, gauge } => {
            if let Some(f) = faces.get_mut(*face as usize) {
                f.transport.gauge = compose_gauge(&f.transport.gauge, gauge);
            }
        }
    }
}

pub(crate) fn unapply_by(faces: &mut [FormFace], by: &PivotBy) {
    match by {
        PivotBy::Word { face, word, side } => {
            if let Some(f) = faces.get_mut(*face as usize) {
                f.transport.word = unpivoted_face(&f.transport.word, word, *side);
            }
        }
        PivotBy::Placement { face, shift } => {
            if let Some(f) = faces.get_mut(*face as usize) {
                f.transport.shift = (
                    &f.transport.shift.0 - &shift.0,
                    &f.transport.shift.1 - &shift.1,
                );
            }
        }
        PivotBy::Hue { face, gauge } => {
            if let Some(f) = faces.get_mut(*face as usize) {
                f.transport.gauge = compose_gauge(&f.transport.gauge, &gauge_invert(gauge));
            }
        }
    }
}

pub fn pivoted_face(g: &Word, w: &Word, side: Side) -> Word {
    match side {
        Side::Right => g.compose(&w.inverse()).normal_form(),
        Side::Left => w.compose(g).normal_form(),
    }
}

/// The face word before that move: the inverse of [`pivoted_face`].
pub fn unpivoted_face(g: &Word, w: &Word, side: Side) -> Word {
    match side {
        Side::Right => g.compose(w).normal_form(),
        Side::Left => w.inverse().compose(g).normal_form(),
    }
}

/// The axis index of a crystal (12.6, A16 step 3): chart → axis slot → M1
/// word. Nested keyed reads, never a hash of the pair.
pub type AxisIndex = Keyed<Keyed<Keyed<Word>>>;

pub(crate) fn axis_slot(axis: Axis) -> [u8; 32] {
    let mut k = [0u8; 32];
    k[0] = axis.to_byte();
    k
}

/// The words a form speaks: its Word vertices'.
pub fn words_of_form(f: &Form) -> Vec<Word> {
    f.vertices
        .iter()
        .filter_map(|v| match v {
            Vertex::Word { word, .. } => Some(word.clone()),
            _ => None,
        })
        .collect()
}

/// The words a held element speaks: a form's Word vertices, or a
/// connection's letters.
pub fn words_of_element(e: &Element) -> Vec<Word> {
    if e.is_sentence() {
        return from_element_form(e).map(|f| words_of_form(&f)).unwrap_or_default();
    }
    e.letters().map(|l| vec![l.spelling.word]).unwrap_or_default()
}

/// Admit words to the axis index under every coordinate their class vector
/// occupies on `chart`.
pub fn admit_words(index: &mut AxisIndex, chart: &stalk::Invariant, words: &[Word]) {
    let ck = chart.key();
    for w in words {
        for axis in ClassVector::of_word(w).coordinates() {
            let ak = axis_slot(axis);
            if index.get(&ck).is_none() {
                index.insert(ck, Keyed::new());
            }
            if let Some(mid) = index.get_mut(&ck) {
                mid.nested_insert(ak, w.key(), w.clone());
            }
        }
    }
}

/// Retract words from the axis index (shed, retire).
pub fn retract_words(index: &mut AxisIndex, chart: &stalk::Invariant, words: &[Word]) {
    let ck = chart.key();
    let Some(mid) = index.get_mut(&ck) else {
        return;
    };
    for w in words {
        for axis in ClassVector::of_word(w).coordinates() {
            if let Some(bucket) = mid.get_mut(&axis_slot(axis)) {
                bucket.remove(&w.key());
            }
        }
    }
}

/// The fiber of the axis index at (chart, axis).
pub fn on_axis_of(index: &AxisIndex, chart: &stalk::Invariant, axis: Axis) -> Vec<Word> {
    index
        .get(&chart.key())
        .and_then(|mid| mid.get(&axis_slot(axis)))
        .map(|bucket| bucket.fiber(&crate::network::nerve::ROOT).map(|(_, w)| w.clone()).collect())
        .unwrap_or_default()
}

/// One candidate pivot: a held word that occupies the axis, written on the
/// chord on one side. Two candidates whose face words are one vertex are one.
struct Candidate {
    axis: Axis,
    by: PivotBy,
}

/// Hold a vortex cell by its after class, then by the discharging word's
/// rotor A1 key. Collapse reads `(class, r.key())`.
pub fn vortex_put(v: &mut Keyed<Keyed<PivotCertificate>>, c: PivotCertificate) {
    let after = c.after;
    let k = c.by.move_key();
    v.nested_insert(after, k, c);
}

/// Dominance certifies (law 5): the lane-wise minimum of the candidates'
/// μ⃗ is computed once; a candidate whose μ⃗ equals that minimum is the
/// discharge. Several that match, or none that match (incomparable lanes),
/// are a vortex. No pairwise pass.
pub(crate) fn select_discharge(cands: &[PivotCertificate]) -> Discharge {
    if cands.is_empty() {
        return Discharge::NoneDischarged;
    }
    let measured: Vec<[u64; 4]> = cands
        .iter()
        .filter_map(|c| match c.mu {
            Mu::Measured { after, .. } => Some(after),
            Mu::Unmeasured => None,
        })
        .collect();
    if measured.is_empty() {
        return if cands.len() == 1 {
            Discharge::One(cands[0].clone())
        } else {
            Discharge::Vortex(cands.to_vec())
        };
    }
    if measured.len() != cands.len() {
        return Discharge::Vortex(cands.to_vec());
    }
    match crate::economy::dominant(&measured) {
        Some(i) => Discharge::One(cands[i].clone()),
        None => Discharge::Vortex(cands.to_vec()),
    }
}
