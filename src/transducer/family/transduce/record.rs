// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! ACT — THE INCIDENCE RECORD of two frames: which bands of the input's
//! bodies meet which bands of the output's on the one turn circle of each
//! axis, read by integer cross-multiplication under the re-period of each
//! circle, never by writing a cell of a common lattice.

use super::{Band, Enclosed, Enclosure, TransductionRefusal, Turn};
use crate::transducer::family::perceive::Grid;

/// ONE INCIDENCE: band `band_in` of the input frame and band `band_out` of
/// the output frame overlap with positive length on both circles. The hues
/// are the two bands' own, carried so a reader of the record never reaches
/// back into a raster. The overlap's turns are read off the two bands.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Incidence {
    pub band_in: usize,
    pub band_out: usize,
    pub before: u8,
    pub after: u8,
}

/// THE TRANSDUCTION OF A PAIR: both frames lifted to the bodies' bands,
/// and every incidence between them under the re-period, in input band
/// order and, within it, in the order the output's rows and columns meet
/// it — an order of content.
#[derive(Clone, Debug)]
pub struct Transduction {
    pub from: Enclosed,
    pub to: Enclosed,
    pub incidences: Vec<Incidence>,
}

/// THE ARCS OF ONE CIRCLE'S MERGE: the boundaries of periods `a` and `b`
/// merged in turn order are `a + 1` and `b + 1` turns of which `gcd(a, b) +
/// 1` coincide (`r·b = r′·a`), so the merged circle has `a + b − gcd(a, b)`
/// arcs. `None` past `usize`.
pub fn arcs(a: usize, b: usize) -> Option<usize> {
    a.checked_add(b)?.checked_sub(num_integer::gcd(a, b))
}

/// THE STROKE A TRANSDUCTION NEEDS, read off the four periods before any
/// band: one read per (input band, output row arc, output band) the record
/// visits, and each such visit holds an arc of the merged row circle and
/// an arc of the merged column circle no other visit holds, so `need =
/// R_h · R_w` bounds the spend, `R` the arcs of each circle's merge
/// ([`arcs`]). `None` on a zero period or past `usize`.
pub fn need(h_in: usize, w_in: usize, h_out: usize, w_out: usize) -> Option<usize> {
    if h_in == 0 || w_in == 0 || h_out == 0 || w_out == 0 {
        return None;
    }
    arcs(h_in, h_out)?.checked_mul(arcs(w_in, w_out)?)
}

/// [`need`] on the periods two grids claim; `None` where either has none.
pub fn need_of(x: &Grid, y: &Grid) -> Option<usize> {
    let p = Enclosure::of(x).ok()?;
    let q = Enclosure::of(y).ok()?;
    need(p.rows, p.cols, q.rows, q.cols)
}

/// The overlap of two arcs, as its two boundary turns.
fn overlap(a: (usize, usize), p: usize, b: (usize, usize), q: usize) -> (Turn, Turn) {
    let (a0, a1, b0, b1) = (Turn::new(a.0, p), Turn::new(a.1, p), Turn::new(b.0, q), Turn::new(b.1, q));
    (if a0 < b0 { b0 } else { a0 }, if a1 < b1 { a1 } else { b1 })
}

/// O1 — THE SHARE A FAMILY CHANGES: the sum over its pairs of the share of
/// each frame its transduction changes ([`Transduction::share`]), exact.
/// `None` where a pair has no enclosure or its record does not read whole
/// under the stroke its periods need. This is the reading μ switches to
/// once the lanes carry rationals; until then it stands beside the count.
pub fn family_share(family: &[(Grid, Grid)]) -> Option<crate::core::Q> {
    let mut share = crate::core::Q::from_integer(0.into());
    for (x, y) in family {
        let mut stroke = crate::physics::Physics::default().stroke_n(need_of(x, y)?);
        share += Transduction::of(x, y, &mut stroke).ok()?.share();
    }
    Some(share)
}

impl Transduction {
    /// THE RECORD OF A PAIR under the caller's stroke, sized by
    /// [`need_of`]. `Empty` or `Ragged` where either frame has no
    /// enclosure; `Spent { need }` where the stroke ends first, with no
    /// record returned.
    pub fn of(x: &Grid, y: &Grid, budget: &mut crate::physics::Budget) -> Result<Transduction, TransductionRefusal> {
        let from = Enclosed::of(x)?;
        let to = Enclosed::of(y)?;
        let (Enclosure { rows: h, cols: w }, Enclosure { rows: h2, cols: w2 }) = (from.enclosure, to.enclosure);
        let need = need(h, w, h2, w2);
        // Each output row arc's bands, in column order: the output's own
        // runs, row by row.
        let mut rows_out: Vec<Vec<(usize, usize, usize)>> = vec![Vec::new(); h2];
        for (i, b) in to.bands.iter().enumerate() {
            for r in b.rows.0..b.rows.1 {
                if let Some(row) = rows_out.get_mut(r) {
                    row.push((b.cols.0, b.cols.1, i));
                }
            }
        }
        for row in &mut rows_out {
            row.sort_unstable();
        }
        let mut incidences = Vec::new();
        for (a, band) in from.bands.iter().enumerate() {
            // The output row arcs that meet the band's row arc with
            // positive length: `⌊r0·h′/h⌋ ≤ r′ < ⌈r1·h′/h⌉`.
            // Both bounds are at most `h′`, so they fit where `h′` does.
            let (Ok(lo), Ok(hi)) = (
                usize::try_from(band.rows.0 as u128 * h2 as u128 / h as u128),
                usize::try_from((band.rows.1 as u128 * h2 as u128).div_ceil(h as u128)),
            ) else {
                continue;
            };
            for r2 in lo..hi {
                let Some(row) = rows_out.get(r2) else { continue };
                // The first run of the row that ends past the band's first
                // column turn; from there every run up to the band's last
                // column turn meets it.
                let start = row.partition_point(|&(_, c1, _)| Turn::new(c1, w2) <= Turn::new(band.cols.0, w));
                for &(c0, _, b) in row.iter().skip(start) {
                    if Turn::new(c0, w2) >= Turn::new(band.cols.1, w) {
                        break;
                    }
                    if !budget.spend() {
                        return Err(TransductionRefusal::Spent { need });
                    }
                    let Some(out) = to.bands.get(b) else { continue };
                    // One incidence per band pair, at the first row arc the
                    // two share.
                    if r2 != lo.max(out.rows.0) {
                        continue;
                    }
                    incidences.push(Incidence { band_in: a, band_out: b, before: band.hue, after: out.hue });
                }
            }
        }
        Ok(Transduction { from, to, incidences })
    }

    /// Is the re-period the identity: both frames of one enclosure?
    pub fn is_planar(&self) -> bool {
        self.from.enclosure.is_same_periods(&self.to.enclosure)
    }

    /// The two bands of an incidence.
    fn bands_of(&self, i: &Incidence) -> Option<(&Band, &Band)> {
        Some((self.from.bands.get(i.band_in)?, self.to.bands.get(i.band_out)?))
    }

    /// The incidences whose hues differ: the change the pair records.
    pub fn changed(&self) -> impl Iterator<Item = &Incidence> {
        self.incidences.iter().filter(|i| i.before != i.after)
    }

    /// O1 — THE SHARE OF THE FRAME THE PAIR CHANGED, exact: the sum over
    /// its changed incidences of the row overlap's length in turns times
    /// the column overlap's, the flat measure of the torus face (the
    /// product of two circles' measures, never a square). Resolution-free;
    /// on one enclosure it is the changed sites over `h·w`.
    pub fn share(&self) -> crate::core::Q {
        let (p, q) = (self.from.enclosure, self.to.enclosure);
        let mut sum = crate::core::Q::from_integer(0.into());
        for i in self.changed() {
            let Some((a, b)) = self.bands_of(i) else { continue };
            let (r0, r1) = overlap(a.rows, p.rows, b.rows, q.rows);
            let (c0, c1) = overlap(a.cols, p.cols, b.cols, q.cols);
            sum += (r1.value() - r0.value()) * (c1.value() - c0.value());
        }
        sum
    }

    /// PROJECT (D3.2 §E) — THE RECORD READ BACK ON THE OUTPUT FRAME: what
    /// the input, re-periodded and nothing else, says against the output,
    /// site by site — the residue of the identity's speech, `(row, col,
    /// before, after)` at every output site whose hues differ. Every band
    /// boundary turn of the input lands on an output boundary or the
    /// projection refuses `OffSite` naming it: a boundary inside an output
    /// compartment would speak two hues there, and none is chosen. On one
    /// enclosure it is the planar coboundary link for link.
    pub fn projected(&self) -> Result<Vec<(i64, i64, u8, u8)>, TransductionRefusal> {
        let (p, q) = (self.from.enclosure, self.to.enclosure);
        let mut landed = Vec::with_capacity(self.from.bands.len());
        for b in &self.from.bands {
            let [r0, r1, c0, c1] = b.turns(&p);
            landed.push(((r0.lands(q.rows)?, r1.lands(q.rows)?), (c0.lands(q.cols)?, c1.lands(q.cols)?)));
        }
        let mut out = Vec::new();
        for i in self.changed() {
            let (Some(&(rows, cols)), Some(b)) = (landed.get(i.band_in), self.to.bands.get(i.band_out)) else { continue };
            let (r0, r1) = (rows.0.max(b.rows.0), rows.1.min(b.rows.1));
            let (c0, c1) = (cols.0.max(b.cols.0), cols.1.min(b.cols.1));
            for r in r0..r1 {
                for c in c0..c1 {
                    out.push((r as i64, c as i64, i.before, i.after));
                }
            }
        }
        out.sort_unstable();
        Ok(out)
    }

    /// THE OUTPUT BODY AN INPUT BODY IS CARRIED ONTO under the re-period:
    /// the one output body its bands meet, where that body meets no other
    /// input body. The frames' bands partition both frames, so then the
    /// body's image IS that body, boundary turn for boundary turn. Any other
    /// meeting — split across bodies, or merged with another — is `Mixed`,
    /// named by the input body.
    pub fn carried(&self, body: usize) -> Result<usize, TransductionRefusal> {
        let mixed = TransductionRefusal::Mixed { body };
        let mut onto: Option<usize> = None;
        for i in &self.incidences {
            if self.from.body_of.get(i.band_in) != Some(&body) {
                continue;
            }
            let Some(&q) = self.to.body_of.get(i.band_out) else { return Err(mixed) };
            match onto {
                None => onto = Some(q),
                Some(held) if held == q => {}
                Some(_) => return Err(mixed),
            }
        }
        let q = onto.ok_or(mixed)?;
        for i in &self.incidences {
            if self.to.body_of.get(i.band_out) == Some(&q) && self.from.body_of.get(i.band_in) != Some(&body) {
                return Err(mixed);
            }
        }
        Ok(q)
    }

    /// Every input body's carriage under the re-period, by body.
    pub fn relations(&self) -> Vec<Result<usize, TransductionRefusal>> {
        (0..self.from.bodies).map(|b| self.carried(b)).collect()
    }
}
