// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! A16 step 4, A18, T21 — THE TRIAL: one candidate applied to a copy of the
//! faces, the class vectors re-read off what it wrote, the rank audited and
//! the bundle measured — and the certificate that states the move, or the
//! refusal that states why there is none.
//!
//! Split from `drive.rs` at the §1.1 1200-line reading bound; the items are
//! moved, not rewritten (§1.6). `pivot_orbit` and `candidates_for` stay with
//! the drive that calls them; the trial and the filing a refused orbit falls
//! through to are one reading and travel together.

use super::drive::{class_vectors_on, holonomy_words_on, orbit_holonomy_on, Prior};
use super::*;
use crate::transducer::sentence::ObstructionRefusal;

/// WHAT ONE MEMBER PATH OF AN ORBIT CLOSES AT: the class its holonomy stands
/// in, the period it closes at, and the hue generator it turns by.
///
/// The three coordinates `FiledOrbit` is written from, minus the chord —
/// which belongs to the ORBIT and not to the path, so it is supplied by the
/// filer and is not re-stated here (law 12).
#[derive(Debug, Clone)]
pub(crate) struct OrbitClass {
    pub(crate) class: crate::network::nerve::coord::Holonomy,
    /// THE TWO PLANE PERIODS (54.7). It was `u32`, the `lcm` of them, and
    /// an `lcm` is an envelope: it cannot tell a planar rotation `[ℓ, 1]`
    /// from an isoclinic `[ℓ, ℓ]`, it reports a dense orbit on the 2-torus
    /// as closed, and it discards the beat two orbits would resonate on.
    /// A plane's `None` is that factor turning forever — A19 spelled it
    /// `0` and the type says it now (rule 11).
    pub(crate) periods: crate::transducer::sentence::ToralPeriod,
    pub(crate) sigma: Vec<(u8, u8)>,
}

impl Form {
    /// ONE CANDIDATE TRIED: its certificate, or its refusal by name (A.17),
    /// returned and never counted — one orbit tried by k candidates is one
    /// fact (law 12), counted once on the drive's demand arm, its kind read
    /// off the orbit's holonomy (78.9).
    pub(crate) fn trial(
        &self,
        idx: usize,
        cand: &Candidate,
        kappa: &ClassVector,
        before: &Prior<'_>,
    ) -> Result<PivotCertificate, ObstructionRefusal> {
        // No orbit, class, seed, row or measure stood to ask: no question.
        let none = ObstructionRefusal::TrialNoQuestion;
        let &Prior { rank_before, crystal, bundle, t_extent, ref mu_before, ref agreements } = before;
        // THE TRIAL OWNS WHAT THE MOVE WRITES AND BORROWS THE REST. A
        // `Word`, `Placement` or `Hue` move writes ONE FACE's transport and
        // nothing else, so the trial owns the faces and reads the orbits
        // they close into; the form's vertices, its indices and its
        // carriers are unchanged and are not copied for a candidate that is
        // about to be refused.
        //
        // Opening an axis writes the form's axis carrier as well, and that
        // carrier is small: the trial owns a copy of it for that arm.
        // THE ORBIT TRIALLED, BY ITS CHORD (AT1): the trial re-reads the
        // orbits off the faces it wrote, and finds this one there by the
        // overlap that closes it, never by its position in either list.
        let chord = self.orbits.get(idx).ok_or(none)?.chord.clone();
        let mut faces = self.faces.clone();
        let mut axes = self.axes.clone();
        apply_by(&mut faces, &cand.by);
        let cycles = Form::cycles_of(&faces, &self.vertices);
        let orbits = cycles.kept.clone();
        // OPENING AN AXIS files the orbit's class against the chord it closes
        // on (I6). A move that does not apply — no orbit on that chord, no
        // class, no seed to re-derive it from — is not a candidate that
        // failed to discharge; there is no trial to read (rule 11).
        if let PivotBy::OpenAxis { face, .. } = &cand.by {
            let lp = orbits.iter().find(|lp| lp.chord == *face).ok_or(none)?;
            // THE TRIAL'S OWN FACES, not the form's. The move has already
            // been applied to `faces` above, so the composite this keys by is
            // the one the orbit carries AFTER the write — which is what
            // `class_vectors_on` reads two lines below to decide whether the
            // orbit discharged. Keyed off `self` the axis would be filed under
            // the holonomy the orbit had before its own opening.
            let held = orbit_holonomy_on(&faces, &self.vertices, lp).ok_or(none)?;
            let class = held.rotor().holonomy_key().ok_or(none)?;
            element::versor_of_word(&held).ok_or(none)?;
            axes.put(&class, Some(face.clone()));
        }
        // 12.6: a pivot on axis x discharges when x reads zero after and no
        // other coordinate moved; opening an axis discharges the ORBIT — it
        // is no longer an orbit on the refined cover (15.5, T14) — so its
        // whole vector reads zero.
        //
        // EVERY MEMBER PATH MUST DISCHARGE. The trial is a write on a FACE,
        // which relates every pair of bodies the face relates, so a move that
        // zeroes the coordinate along one path and leaves it standing on
        // another has not discharged the orbit — it has discharged one reading
        // of it, and the pivot certificate would assert the rest. An orbit with
        // no closed member path has nothing to read and refuses (rule 11).
        // The trial rewrites the faces and reopens the axes; the VERTICES are
        // untouched by it, so the Frame is read off the form's own.
        let after =
            class_vectors_on(&faces, &self.vertices, &axes, orbits.iter().find(|lp| lp.chord == chord).ok_or(none)?);
        if after.is_empty() {
            return Err(none);
        }
        let discharged = after.scoped().all(|(_, kappa_after)| {
            if cand.axis == Axis::Open {
                kappa_after.is_zero()
            } else {
                !kappa_after.occupies(cand.axis) && !kappa_after.rises_against(kappa, cand.axis)
            }
        });
        if !discharged {
            return Err(ObstructionRefusal::TrialNotDischarged);
        }
        // THE CERTIFICATE NAMES ONE CLASS AFTER, so it is only writable where
        // every member path agrees on it. The plane and off-plane coordinates
        // are path-independent; a residue that differs between paths means the
        // orbit has no single class and there is nothing to certify (rule 11).
        let mut agreed: Option<ClassVector> = None;
        for (_, cv) in after.scoped() {
            match &agreed {
                None => agreed = Some(cv.clone()),
                Some(h) if h.class_key() != cv.class_key() => return Err(ObstructionRefusal::TrialPathsDisagree),
                Some(_) => {}
            }
        }
        let kappa_after = agreed.ok_or(none)?;
        // THE RANK AFTER IS READ OVER EVERY ROW, because no move a pivot
        // makes is confined to one orbit.
        //
        // This substituted row `idx` alone for `Word`, `Placement` and
        // `Hue`, on the ground that such a move "writes ONE face's transport,
        // so exactly one orbit's holonomy moves". The premise is false
        // wherever two orbits SHARE that face, which a form of more than one
        // orbit routinely does: the write is on the face, and every orbit
        // running through it reads the carrier that moved.
        //
        // It was already false for `Placement` and `Hue`, which write
        // `p.carriage` and `p.gauge` at every member pair of the face. R13
        // found it for `OpenAxis` — the axis carrier is keyed by CLASS, so
        // one opening opens every orbit of that class — and fixed only that
        // arm. R16 then made it unmissable for `Word`: a word move now
        // writes the EDGE transports as well as the face word, and the edge
        // is exactly the carrier every other orbit through that face reads
        // its class from.
        //
        // MEASURED at d1d84ca, first soak with the whole block live: 50
        // occurrences of `axis Orientation(0) refused: rank moved
        // [1, 0, 0, 0]` — torsion rising by one, with nothing leaving.
        // That is the signature: the trialled orbit's row was substituted
        // and a SIBLING orbit's class, moved by the same write, was still
        // counted at its old value, so the audit saw a class appear and
        // none go. The refusal was correct and the certificate was never
        // written; what was wrong is that the trial could not see its own
        // move.
        //
        // The substitution is removed rather than extended to the shared
        // case, because "which orbits share this face" is a second reader of
        // the thing `orbits_of` already answered, and two readers of one fact
        // is the defect this file keeps finding. The cost is one
        // `orbit_row_on` per orbit per surviving candidate, and R19's
        // eliminator already refuses the candidates that cannot cancel
        // before any of this is reached.
        let mut after_rows: Vec<OrbitRow> = Vec::with_capacity(orbits.len());
        for lp in &orbits {
            after_rows.push(crate::transducer::sentence::orbit_row_on(
                &faces,
                &self.vertices,
                &axes,
                lp,
            ).ok_or(none)?);
        }
        let rank_after = rank_of_rows(&after_rows).ok_or(none)?;
        // T21 (iii): the rank holds, or one unit moves out of torsion, free
        // or off-plane and into axes. Anything else is not this pivot's move
        // and the trial refuses it here, naming what the audit did.
        let rank_move = Rank::moved(rank_before, rank_after);
        // The audit reads every row, so a sibling on the shared face carried
        // to a new class is a move of this pivot too (R26).
        if let RankMove::Inadmissible { .. } = rank_move {
            return Err(ObstructionRefusal::TrialRankInadmissible);
        }
        let mu = match mu_before {
            None => MeasuredMu::Unmeasured,
            Some((pairs_before, before)) => {
                let (pairs_before, before) = (pairs_before, *before);
                // THE BUNDLE IS SPOKEN THROUGH A WHOLE FORM, so this is
                // where one is built — for a candidate that has already
                // discharged its axis and held the rank, which is the few.
                // Every candidate paid for it before, including the ones
                // refused two readings above.
                let trial_form = Form {
                    faces: faces.clone(),
                    orbits: orbits.clone(),
                    axes: axes.clone(),
                    ..self.clone()
                };
                let (pairs_after, after) = crate::transducer::family::bundle_pressure(
                    &trial_form,
                    bundle.ok_or(none)?,
                    crystal,
                    t_extent,
                )
                .ok_or(none)?;
                // 12.6: a trial refuses a candidate that raises any lane on
                // ANY pair, and the reading names the pairs. Two readings of
                // different lengths are not the same bundle and are not
                // comparable, which is also a refusal — of the comparison.
                let raised =
                    crate::transducer::family::pairs_raised(pairs_before, &pairs_after).ok_or(none)?;
                if !raised.is_empty() {
                    return Err(ObstructionRefusal::TrialLaneRose);
                }
                // 12.6, T21 (iv): a candidate that RAISES any lane is
                // refused, read per lane and named. Strictness is not
                // asked: a pivot that moves no lane is certified by its
                // closure, not by a fall.
                let reading = crate::economy::lane_moves(&after, &before);
                let risen = crate::economy::lanes_risen(&reading);
                if !risen.is_empty() {
                    return Err(ObstructionRefusal::TrialLaneRose);
                }
                MeasuredMu::Measured { before, after }
            }
        };
        // 78.46(4′), LAW 10 — NO OTHER COORDINATE ROSE, read on the cells:
        // a move that splits two circuits of one anastomosis that agreed —
        // the pair composite the identity before and non-trivial after —
        // raised a coordinate it did not act on, and has not closed the
        // cell. A pair already non-trivial before does not refuse it.
        let after_agreements = super::Agreements::read(self, &faces, &cycles, &mut super::PerturbTally::default());
        if agreements.split_by(&after_agreements) {
            return Err(ObstructionRefusal::TrialUnclosed);
        }
        Ok(PivotCertificate {
            chord,
            axis: cand.axis,
            before: kappa.class_key(),
            after: kappa_after.class_key(),
            rank_move,
            mu,
            by: cand.by.clone(),
        })
    }

    /// FILE THE ORBIT'S TORSION UNDER EVERY CLASS ITS MEMBER PATHS CARRY.
    ///
    /// Torsion is filed under the holonomy's CLASS (16.17, 15.4), and the
    /// holonomy of a path is its edge composite where it has one, so an orbit
    /// flat on its words and curved on its edges files the edges' classes.
    /// The period is finite at `ℓ`, `0` for infinite order, A19's spelling
    /// of it (52.3).
    ///
    /// THE PERIOD IS DERIVED HERE AND NO LONGER HANDED IN (54.3, as ruled
    /// 2026-09-20). It was a parameter, and `drive.rs` passed `hw.order()`
    /// — THE ROTOR'S ORDER ALONE. A 90° turn carried along a
    /// translation has the same rotor order as one about a fixed point and
    /// is not the same class: the screw does not return after four turns,
    /// it SLIPS, and it generates a non-compact ℤ rather than a compact
    /// ℤ/4ℤ. Filed at four, a theorem stood on the orbit that the orbit does
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
    /// One class per member path, not one for the orbit: an orbit whose faces
    /// relate several pairs of bodies has one composite per path through
    /// them, and each is its own obstruction. A path that carries no versor
    /// has no class and is skipped; an orbit where no path files returns an
    /// EMPTY listing and stays open, which is what `torsion open` reports.
    ///
    /// Returns THE PERIODS FILED, each once, in coordinate order — so the
    /// caller reports what was filed rather than what it asked for. Two
    /// member paths of one orbit may close at two periods, and a single
    /// number would have to pick one of them by position (rule 15).
    pub(crate) fn file_orbit(&mut self, idx: usize) -> Vec<u64> {
        let Some(lp) = self.orbits.get(idx) else {
            return Vec::new();
        };
        let chord = lp.chord.clone();
        let mut filed: crate::network::nerve::Keyed<u64> = crate::network::nerve::Keyed::new();
        for OrbitClass { class, periods, sigma } in self.orbit_classes(lp) {
            self.torsion.put(&class, FiledOrbit { sigma, periods, chord: Closes::On(chord.clone()) });
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
            #[expect(
                clippy::manual_unwrap_or_default,
                reason = "A19 — a spectrum that does not close is spelled 0 on the ledger; the zero is that spelling, not a default standing in for a reading"
            )]
            let u = match periods.yield_units() {
                Some(units) => units,
                None => 0,
            };
            filed.insert(u.to_be_bytes(), u);
        }
        filed.scoped().map(|(_, ell)| *ell).collect()
    }

    /// WHAT EACH MEMBER PATH OF AN ORBIT CLOSES AT: its class, the period it
    /// closes at, and the hue generator it turns by — one row per path that
    /// carries a class AND a measured transport.
    ///
    /// ONE READER FOR THREE CALLERS (law 12). [`file_orbit`] writes these
    /// rows onto `Form.torsion`, `Form::coheres` asks whether they are all
    /// compact, and `Form::is_cauterized` asks whether what is filed agrees
    /// with them. Those three asked three different questions of one orbit
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
    pub(crate) fn orbit_classes(&self, lp: &Orbit) -> Vec<OrbitClass> {
        let words = holonomy_words_on(&self.faces, &self.vertices, lp);
        // THE GENERATOR FILED IS THE HUE MAP, read at the same member path
        // the word was read at. `FiledOrbit::sigma` says what it is in its
        // own line — "the hue-map generator: the permutation one step
        // around the orbit" — and `torsion_witness::witness_cycle` runs round it
        // with `apply_hue_map` against a thought's hue assignment.
        //
        // This wrote `lp.vertices.windows(2)` into it: the orbit's VERTEX
        // PATH, truncated `u16` to `u8`, filed as a permutation of hues.
        // Every witness of a cycle filed here ran round vertex indices as
        // recolourings, and a form of more than 255 vertices aliased them
        // first. `extract.rs` files `e.gauge` into the same field, which
        // is the reading; this site was the only one writing another
        // object into it.
        let transports = orbit_transport_on(&self.faces, &self.vertices, lp);
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
                crate::transducer::sentence::CycleType::Identity,
            ) {
                crate::transducer::cech::Fill::Congruent(..) => w.rotor().toral_period(),
                _ => crate::transducer::sentence::ToralPeriod { plane_1: None, plane_2: None },
            };
            out.push(OrbitClass { class, periods, sigma: transport.gauge.clone() });
        }
        out
    }

    /// 10.3 AS A COCYCLE AND NOT AS FLATNESS: does every member path of
    /// this orbit close at a COMPACT period?
    ///
    /// An orbit whose holonomy is the identity is flat. An orbit whose holonomy
    /// is a finite-order rotation about a fixed point is NOT flat and is
    /// still closed: it returns after ℓ turns, and what it returns by is a
    /// `ℤ/ℓℤ` class the form carries. Treating the second as an unclosed
    /// form is what made curvature illegal — see `Form::coheres`.
    ///
    /// An orbit with NO classifiable path answers false: it is unmeasured,
    /// not compact (rule 11). A path that closes at `0` is non-compact — a
    /// screw or a dilation, a free generator — and that is an obstruction
    /// the form has genuinely not resolved, so it answers false too. The
    /// free part is what a demand is for (52.3); only the torsion part
    /// closes.
    pub(crate) fn orbit_is_congruent(&self, lp: &Orbit) -> bool {
        let rows = self.orbit_classes(lp);
        // BOTH PLANES MUST CLOSE. It read `period >= 1` on the `lcm`, which
        // is true whenever EITHER factor is a root of unity — so an orbit with
        // one plane turning forever read as compact. An orbit closes when its
        // orbit on the 2-torus closes, which is both planes (54.7).
        !rows.is_empty() && rows.iter().all(|r| r.periods.closes())
    }
}
