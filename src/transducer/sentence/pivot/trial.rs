// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! A16 step 4, A18, T21 — THE TRIAL: one candidate applied to a copy of the
//! faces, the class vectors re-read off what it wrote, the rank audited and
//! the bundle measured — and the certificate that states the move, or the
//! refusal that states why there is none.
//!
//! Split from `drive.rs` at the §1.1 1200-line reading bound; the items are
//! moved, not rewritten (§1.6). `pivot_loop` and `candidates_for` stay with
//! the drive that calls them; the trial and the filing a refused loop falls
//! through to are one reading and travel together.

use super::drive::{class_vectors_on, holonomy_words_on, loop_holonomy_on, Prior};
use super::*;

/// WHAT ONE MEMBER PATH OF A LOOP CLOSES AT: the class its holonomy stands
/// in, the period it closes at, and the hue generator it turns by.
///
/// The three coordinates `FiledLoop` is written from, minus the chord —
/// which belongs to the LOOP and not to the path, so it is supplied by the
/// filer and is not re-stated here (law 12).
#[derive(Debug, Clone)]
pub(crate) struct LoopClass {
    pub(crate) class: crate::network::nerve::coord::Holonomy,
    /// THE TWO PLANE PERIODS (54.7). It was `u32`, the `lcm` of them, and
    /// an `lcm` is an envelope: it cannot tell a planar rotation `[ℓ, 1]`
    /// from an isoclinic `[ℓ, ℓ]`, it reports a dense orbit on the 2-torus
    /// as closed, and it discards the beat two loops would resonate on.
    /// A plane's `None` is that factor turning forever — A19 spelled it
    /// `0` and the type says it now (rule 11).
    pub(crate) periods: crate::transducer::sentence::ToralPeriod,
    pub(crate) sigma: Vec<(u8, u8)>,
}

impl Form {
    pub(crate) fn trial(
        &self,
        idx: usize,
        cand: &Candidate,
        kappa: &ClassVector,
        before: &Prior<'_>,
    ) -> Option<PivotCertificate> {
        let &Prior { rank_before, crystal, bundle, t_extent, ref mu_before } = before;
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
        let loops = Form::loops_of(&faces, &self.vertices);
        // OPENING AN AXIS files the loop's class against the chord it closes
        // on (I6). A move that does not apply — no loop on that chord, no
        // class, no seed to re-derive it from — is not a candidate that
        // failed to discharge; there is no trial to read (rule 11).
        if let PivotBy::OpenAxis { face, .. } = &cand.by {
            let lp = loops.iter().find(|lp| lp.chord == *face)?;
            // THE TRIAL'S OWN FACES, not the form's. The move has already
            // been applied to `faces` above, so the composite this keys by is
            // the one the loop carries AFTER the write — which is what
            // `class_vectors_on` reads two lines below to decide whether the
            // loop discharged. Keyed off `self` the axis would be filed under
            // the holonomy the loop had before its own opening.
            let class = loop_holonomy_on(&faces, &self.vertices, lp)?.rotor().holonomy_key()?;
            element::versor_of_word(&loop_holonomy_on(&faces, &self.vertices, lp)?)?;
            axes.put(&class, *face);
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
                !kappa_after.occupies(cand.axis) && !kappa_after.rises_against(kappa, cand.axis)
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
                Some(h) if h.class_key() != cv.class_key() => return None,
                Some(_) => {}
            }
        }
        let kappa_after = agreed?;
        // THE RANK AFTER IS READ OVER EVERY ROW, because no move a pivot
        // makes is confined to one loop.
        //
        // This substituted row `idx` alone for `Word`, `Placement` and
        // `Hue`, on the ground that such a move "writes ONE face's transport,
        // so exactly one loop's holonomy moves". The premise is false
        // wherever two loops SHARE that face, which a form of more than one
        // loop routinely does: the write is on the face, and every loop
        // running through it reads the carrier that moved.
        //
        // It was already false for `Placement` and `Hue`, which write
        // `p.carriage` and `p.gauge` at every member pair of the face. R13
        // found it for `OpenAxis` — the axis carrier is keyed by CLASS, so
        // one opening opens every loop of that class — and fixed only that
        // arm. R16 then made it unmissable for `Word`: a word move now
        // writes the EDGE transports as well as the face word, and the edge
        // is exactly the carrier every other loop through that face reads
        // its class from.
        //
        // MEASURED at d1d84ca, first soak with the whole block live: 50
        // occurrences of `axis Orientation(0) refused: rank moved
        // [1, 0, 0, 0]` — torsion rising by one, with nothing leaving.
        // That is the signature: the trialled loop's row was substituted
        // and a SIBLING loop's class, moved by the same write, was still
        // counted at its old value, so the audit saw a class appear and
        // none go. The refusal was correct and the certificate was never
        // written; what was wrong is that the trial could not see its own
        // move.
        //
        // The substitution is removed rather than extended to the shared
        // case, because "which loops share this face" is a second reader of
        // the thing `loops_of` already answered, and two readers of one fact
        // is the defect this file keeps finding. The cost is one
        // `loop_row_on` per loop per surviving candidate, and R19's
        // eliminator already refuses the candidates that cannot cancel
        // before any of this is reached.
        let mut after_rows: Vec<LoopRow> = Vec::with_capacity(loops.len());
        for lp in &loops {
            after_rows.push(crate::transducer::sentence::loop_row_on(
                &faces,
                &self.vertices,
                &axes,
                lp,
            )?);
        }
        let rank_after = rank_of_rows(&after_rows)?;
        // T21 (iii): the rank holds, or one unit moves out of torsion, free
        // or off-plane and into axes. Anything else is not this pivot's move
        // and the trial refuses it here, naming what the audit did.
        let rank_move = Rank::moved(rank_before, rank_after);
        if let RankMove::Inadmissible { delta } = rank_move {
            // WHICH ROWS MOVED, and to what (§8.1, measurement only). The
            // delta says a coordinate changed and never which loop did it,
            // so `[1, 0, 0, 0]` — a torsion class appearing with none
            // leaving — reads the same whether the trialled loop acquired a
            // class the form did not carry or a SIBLING on the shared face
            // was carried to one.
            //
            // R26 removed the single-row substitution on exactly this
            // suspicion and the refusal survived it: 50 occurrences before,
            // 5 in the first window after, against a genome a third the
            // size. So the audit is now reading every row and still sees a
            // class appear, which makes it a real move and not bookkeeping.
            // These two lines name the rows so the next reading does not
            // have to guess again.
            //
            // The count of DISTINCT torsion classes is what `rank_of_rows`
            // counts since R14, so the rows are printed as their classes and
            // not as their indices — an index says which loop, and the
            // question is which class.
            // THE LABEL MUST SEPARATE WHAT THE AUDIT SEPARATES. Two bytes of
            // a class key stood here and two DIFFERENT torsion classes
            // printed identically: `rows before ["t0601","t0601"] after
            // ["t0601","t0601"]` against a delta of `[1, 0, 0, 0]`, 87 times
            // — rows that looked unchanged beside an audit that said a class
            // had appeared. The audit was right and the label was blind.
            //
            // `rank_of_rows` counts DISTINCT classes, so a label that cannot
            // tell two apart cannot say what the audit did.
            //
            // 54.7 — IT SAYS THE CLASS, NOT A DIGEST OF ITS KEY. It took
            // the first two and last two bytes of the key and hoped they
            // differed, which is a hash of a coordinate standing in for the
            // coordinate. The class IS `(I₁, I₂, kind)`; two classes differ
            // exactly when those differ, and the line now shows them.
            let tag = |k: &crate::network::nerve::coord::Holonomy| -> String {
                format!("{:?}/{}/{}", k.kind, k.i1, k.i2)
            };
            let cls = |rs: &[LoopRow]| -> Vec<String> {
                rs.iter()
                    .map(|r| match r {
                        LoopRow::Closed => "closed".to_string(),
                        LoopRow::Torsion(k) => format!("t{}", tag(k)),
                        LoopRow::Free(_) => "free".to_string(),
                        LoopRow::OffPlane(k) => format!("o{}", tag(k)),
                        LoopRow::Opened(k) => format!("x{}", tag(k)),
                    })
                    .collect()
            };
            // A form whose rows do not read has no "before" to print, and an
            // empty list is not that reading — it is the reading of a form
            // with no loops (rule 11).
            match self.loop_rows() {
                Some(before) => eprintln!(
                    "  pivot loop {idx} rows before {:?} after {:?}",
                    cls(&before),
                    cls(&after_rows)
                ),
                None => eprintln!(
                    "  pivot loop {idx} rows before unmeasured after {:?}",
                    cls(&after_rows)
                ),
            }
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
    /// holonomy of a path is its edge composite where it has one, so a loop
    /// flat on its words and curved on its edges files the edges' classes.
    /// The period is finite at `ℓ`, `0` for infinite order, A19's spelling
    /// of it (52.3).
    ///
    /// THE PERIOD IS DERIVED HERE AND NO LONGER HANDED IN (54.3, as ruled
    /// 2026-09-20). It was a parameter, and `drive.rs` passed `hw.order()`
    /// — THE ROTOR'S ORDER ALONE. A quarter turn carried along a
    /// translation has the same rotor order as one about a fixed point and
    /// is not the same class: the screw does not return after four turns,
    /// it WALKS, and it generates a non-compact ℤ rather than a compact
    /// ℤ/4ℤ. Filed at four, a theorem stood on the loop that the loop does
    /// not satisfy.
    ///
    /// [`crate::transducer::cech::classify`] is the engine's one reader of
    /// that distinction: it solves for the transport's fixed point exactly
    /// (`turn_base_point`) and answers `Congruent` only where there is one,
    /// so a rotation about ANY point is congruent at its rotor's order — it
    /// is conjugate to that rotor — and a screw or a dilation is not
    /// congruent at all. A path whose holonomy has no fixed point files at
    /// `0`: non-compact, which is A19's infinite order and exactly what it
    /// is. Nothing is discarded (law 7) and nothing is filed at a period it
    /// does not close at (rule 11).
    ///
    /// ONE READER, TWO CALLERS. The court reaches this through the pivot
    /// stroke and `from_address` reaches it while rebuilding a form from
    /// its key, because `Form.torsion` is NOT in the key: it is a theorem
    /// the form's own incidence proves, and law 12 says it is re-derived
    /// rather than stored beside the coordinates that determine it.
    ///
    /// One class per member path, not one for the loop: a loop whose faces
    /// relate several pairs of bodies has one composite per path through
    /// them, and each is its own obstruction. A path that carries no versor
    /// has no class and is skipped; a loop where no path files returns an
    /// EMPTY listing and stays open, which is what `torsion open` reports.
    ///
    /// Returns THE PERIODS FILED, each once, in coordinate order — so the
    /// caller reports what was filed rather than what it asked for. Two
    /// member paths of one loop may close at two periods, and a single
    /// number would have to pick one of them by position (rule 15).
    pub(crate) fn file_loop(&mut self, idx: usize) -> Vec<u32> {
        let Some(lp) = self.loops.get(idx) else {
            return Vec::new();
        };
        let chord = lp.chord;
        let mut filed: crate::network::nerve::Keyed<u32> = crate::network::nerve::Keyed::new();
        for LoopClass { class, periods, sigma } in self.loop_classes(lp) {
            self.torsion.put(&class, FiledLoop { sigma, periods, chord });
            // THE YIELD IS ACROSS BOTH PLANES: `(ℓ₁ − 1) + (ℓ₂ − 1)`, not
            // `(lcm − 1)`, which is neither plane's labour.
            //
            // A SPECTRUM THAT DOES NOT CLOSE YIELDS ZERO AND IS STILL
            // FILED. The insert above happened — the class was measured and
            // written down — and A19 spells a non-closing order `0` on the
            // ledger. Dropping the entry would make the FILING invisible to
            // every reader of this list, including the `TorsionFiled` snap
            // that says a free part was filed beside its demand; a thing
            // measured and then not written down is 53.6 item 2's own
            // refusal (rule 11).
            let u = periods
                .yield_units()
                .map_or(0, |units| u32::try_from(units).unwrap_or(u32::MAX));
            filed.insert(u.to_be_bytes(), u);
        }
        filed.scoped().map(|(_, ell)| *ell).collect()
    }

    /// WHAT EACH MEMBER PATH OF A LOOP CLOSES AT: its class, the period it
    /// closes at, and the hue generator it turns by — one row per path that
    /// carries a class AND a measured transport.
    ///
    /// ONE READER FOR THREE CALLERS (law 12). [`file_loop`] writes these
    /// rows onto `Form.torsion`, `Form::coheres` asks whether they are all
    /// compact, and `Form::is_cauterized` asks whether what is filed agrees
    /// with them. Those three asked three different questions of one loop
    /// before: the filer took its period as a parameter, `coheres` read
    /// `lp.holonomy` and never looked at a carriage at all, and
    /// `is_cauterized` read `lp.holonomy.order()` — the rotor's order with
    /// no fixed-point test — so a screw was filed at 0 by the first and
    /// demanded at 4 by the third.
    ///
    /// The period is `0` where the transport has no fixed point: a screw
    /// generates a non-compact ℤ and closes at nothing, which is A19's
    /// spelling of infinite order. See
    /// [`crate::transducer::cech::classify`].
    pub(crate) fn loop_classes(&self, lp: &Loop) -> Vec<LoopClass> {
        let words = holonomy_words_on(&self.faces, &self.vertices, lp);
        // THE GENERATOR FILED IS THE HUE MAP, read at the same member path
        // the word was read at. `FiledLoop::sigma` says what it is in its
        // own line — "the hue-map generator: the permutation one step
        // around the loop" — and `torsion_witness::witness_cycle` walks it
        // with `apply_hue_map` against a thought's hue assignment.
        //
        // This wrote `lp.vertices.windows(2)` into it: the loop's VERTEX
        // PATH, truncated `u16` to `u8`, filed as a permutation of hues.
        // Every witness of a cycle filed here walked vertex indices as
        // recolourings, and a form of more than 255 vertices aliased them
        // first. `extract.rs` files `e.gauge` into the same field, which
        // is the reading; this site was the only one writing another
        // object into it.
        let transports = loop_transport_on(&self.faces, &self.vertices, lp);
        let mut out = Vec::new();
        for (path, w) in words.scoped() {
            let Some(class) = w.rotor().holonomy_key() else {
                continue;
            };
            // A path with no transport has no gauge to file. An empty
            // listing here would be a placeholder over an unmeasured
            // value, which is the defect `sigma` already carried (rule
            // 11); a path whose transport IS measured and whose gauge is
            // empty files that, because moving no hue is a reading.
            let Some(transport) = transports.get(&path) else {
                continue;
            };
            // THE SPECTRUM THIS PATH CLOSES ON, read from the holonomy's
            // ROTOR AND ITS CARRIAGE together (54.7). A path with no fixed
            // point is non-compact and neither plane closes.
            //
            // `Fill::Congruent(_, _, ell)` carries the `lcm`; the two plane
            // periods are the rotor's own and are read from it directly, so
            // the envelope is never the thing stored.
            let periods = match crate::transducer::cech::classify(
                &w.rotor(),
                &transport.carriage,
                crate::transducer::sentence::CycleType::of(&[]),
            ) {
                crate::transducer::cech::Fill::Congruent(..) => w.rotor().toral_period(),
                _ => crate::transducer::sentence::ToralPeriod { plane_1: None, plane_2: None },
            };
            out.push(LoopClass { class, periods, sigma: transport.gauge.clone() });
        }
        out
    }

    /// 10.3 AS A COCYCLE AND NOT AS FLATNESS: does every member path of
    /// this loop close at a COMPACT period?
    ///
    /// A loop whose holonomy is the identity is flat. A loop whose holonomy
    /// is a finite-order rotation about a fixed point is NOT flat and is
    /// still closed: it returns after ℓ turns, and what it returns by is a
    /// `ℤ/ℓℤ` class the form carries. Treating the second as an unclosed
    /// form is what made curvature illegal — see `Form::coheres`.
    ///
    /// A loop with NO classifiable path answers false: it is unmeasured,
    /// not compact (rule 11). A path that closes at `0` is non-compact — a
    /// screw or a dilation, a free generator — and that is an obstruction
    /// the form has genuinely not resolved, so it answers false too. The
    /// free part is what a demand is for (52.3); only the torsion part
    /// closes.
    pub(crate) fn loop_is_congruent(&self, lp: &Loop) -> bool {
        let rows = self.loop_classes(lp);
        // BOTH PLANES MUST CLOSE. It read `period >= 1` on the `lcm`, which
        // is true whenever EITHER factor is a root of unity — so a loop with
        // one plane turning forever read as compact. A loop closes when its
        // orbit on the 2-torus closes, which is both planes (54.7).
        !rows.is_empty() && rows.iter().all(|r| r.periods.closes())
    }
}
