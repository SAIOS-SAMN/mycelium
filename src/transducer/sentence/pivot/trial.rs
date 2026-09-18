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
        let loops = Form::loops_of(&faces, self.vertices.len());
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
            let class = loop_holonomy_on(&faces, &self.vertices, lp)?.rotor().class_key()?;
            element::versor_of_word(&loop_holonomy_on(&faces, &self.vertices, lp)?)?;
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
            // tell two apart cannot say what the audit did. The key's own
            // tail is taken as well as its head: a class key leads with the
            // conjugacy invariants, which two finite classes of one order
            // share, and differs further along.
            let tag = |k: &crate::transducer::address::Address| -> String {
                let b = k.as_bytes();
                let n = b.len();
                match n >= 4 {
                    true => format!(
                        "{:02x}{:02x}..{:02x}{:02x}",
                        b[0],
                        b[1],
                        b[n - 2],
                        b[n - 1]
                    ),
                    false => format!("{:02x?}", b),
                }
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
    /// The period is the order the caller read: finite at `ℓ`, `0` for
    /// infinite order, A19's spelling of it (52.3).
    ///
    /// One class per member path, not one for the loop: a loop whose faces
    /// relate several pairs of bodies has one composite per path through
    /// them, and each is its own obstruction. A path that carries no versor
    /// has no class and is skipped; a loop where no path files returns
    /// `false` and stays open, which is what `torsion open` reports.
    pub(crate) fn file_loop(&mut self, idx: usize, ell: u32) -> bool {
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
