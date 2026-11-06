// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! ACT — THE INCIDENCE RECORD of two frames of one rank `n`: which bands of
//! the input's bodies meet which bands of the output's, on the one turn
//! circle of each phase, read by integer cross-multiplication under the
//! re-period of each phase, never by writing a cell of a common lattice.

use super::{Band, Enclosed, Enclosure, TransductionRefusal, Turn};
use crate::transducer::base::Site;
use crate::transducer::sample::{Sample, Symbol};

/// ONE INCIDENCE: band `band_in` of the input frame and band `band_out` of
/// the output frame overlap with positive length on every phase. The
/// symbols are the two bands' own, read off the record, so a reader never
/// reaches back into a sample. The overlap's turns are read off the two
/// bands.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Incidence {
    pub band_in: usize,
    pub band_out: usize,
}

/// ONE SITE OF THE OUTPUT FRAME WHERE THE PROJECTED RECORD CHANGES: the
/// site, the symbol the input's speech holds there and the symbol observed.
/// Written only by projection, which is speech.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SiteChange {
    site: Site,
    before: Vec<u8>,
    after: Vec<u8>,
}

impl SiteChange {
    /// The output site.
    pub fn site(&self) -> &Site {
        &self.site
    }

    /// The symbol the input's speech holds at the site.
    pub fn before(&self) -> Symbol<'_> {
        Symbol::new(&self.before)
    }

    /// The symbol observed at the site.
    pub fn after(&self) -> Symbol<'_> {
        Symbol::new(&self.after)
    }
}

/// THE TRANSDUCTION OF A PAIR: both frames lifted to the bodies' bands,
/// and every incidence between them under the re-period, in input band
/// order and, within it, in the order the output's slices along the first
/// phase meet it and, within a slice, the canonical order of the output's
/// bands: an order of content.
#[derive(Clone, Debug)]
pub struct Transduction {
    pub from: Enclosed,
    pub to: Enclosed,
    pub incidences: Vec<Incidence>,
}

/// THE ARCS OF ONE PHASE'S MERGE: the boundaries of periods `a` and `b`
/// merged in turn order are `a + 1` and `b + 1` turns of which `gcd(a, b) +
/// 1` coincide (`r·b = r′·a`), so the merged circle has `a + b − gcd(a, b)`
/// arcs. `None` past `usize`.
pub fn arcs(a: usize, b: usize) -> Option<usize> {
    a.checked_add(b)?.checked_sub(num_integer::gcd(a, b))
}

/// THE STROKE A TRANSDUCTION NEEDS, read off the two enclosures before any
/// band: one read per (input band, output slice arc, output band) the
/// record visits, and each such visit holds an arc of every phase's merge
/// no other visit holds, so `need = Π R_k` bounds the spend, `R_k` the arcs
/// of phase `k`'s merge ([`arcs`]). `None` where the ranks differ or past
/// `usize`.
pub fn need(from: &Enclosure, to: &Enclosure) -> Option<usize> {
    if from.rank() != to.rank() {
        return None;
    }
    from.periods().iter().zip(to.periods()).try_fold(1usize, |n, (&p, &q)| n.checked_mul(arcs(p, q)?))
}

/// [`need`] on the enclosures two samples present.
pub fn need_of(x: &Sample, y: &Sample) -> Option<usize> {
    need(&Enclosure::of(x), &Enclosure::of(y))
}

/// The overlap of two arcs, as its two boundary turns.
fn overlap(a: (usize, usize), p: usize, b: (usize, usize), q: usize) -> (Turn, Turn) {
    let (a0, a1, b0, b1) = (Turn::of(a.0, p), Turn::of(a.1, p), Turn::of(b.0, q), Turn::of(b.1, q));
    (if a0 < b0 { b0 } else { a0 }, if a1 < b1 { a1 } else { b1 })
}

/// An output band in one slice of the first phase: its spans past the first
/// phase, then its index.
type Located = (Vec<(usize, usize)>, usize);

/// Do two arcs of periods `p` and `q` overlap with positive length?
fn meets(a: (usize, usize), p: usize, b: (usize, usize), q: usize) -> bool {
    Turn::of(a.0, p) < Turn::of(b.1, q) && Turn::of(b.0, q) < Turn::of(a.1, p)
}

/// O1 — THE SHARE A FAMILY CHANGES: the sum over its pairs of the share of
/// each frame its transduction changes ([`Transduction::share`]), exact.
/// `None` where a pair's frames differ in rank or its record does not read
/// whole under the stroke its periods need. Per lane it is
/// [`Transduction::lane_shares`]. Read beside the mind's count of a family it
/// takes in (`family_magnitude`), on the mind's observe reading.
pub fn family_share<'a>(family: impl IntoIterator<Item = (&'a Sample, &'a Sample)>) -> Option<crate::core::Q> {
    let mut share = crate::core::Q::from_integer(0.into());
    for (x, y) in family {
        let mut stroke = crate::physics::Physics::default().stroke_n(need_of(x, y)?);
        share += Transduction::of(x, y, &mut stroke).ok()?.share();
    }
    Some(share)
}

/// O1 (ruled 2026-11-01) — A CHAIN'S MEASURE AS μ, A SHARE OF THE FRAME IT
/// IS READ ON: S (sites changed between ground and painted) and G (sites
/// changed from one painted symbol to another) are site counts, each over
/// the frame's measure, the product of its extents at its rank
/// ([`Enclosure::measure`]), so one changed site of a frame of 900 sites is
/// `1/900` of it; H is a shell count, topological and already
/// resolution-free, and stands as its count `n/1`; T stands as its count
/// too, and a chain carries none (the deflation reads T off the flux,
/// `deflation::form_deflation_read`). With no frame (`None`: a source that
/// presents no sample) the measure stays `n/1`: a discharge with no frame
/// keeps its own measure (ruling 3).
pub fn share_of_enclosure(counts: [u64; 4], frame: Option<&Sample>) -> crate::economy::Mu {
    let whole = crate::economy::Mu::counts(counts);
    let Some(sample) = frame else { return whole };
    match Enclosure::of(sample).measure().and_then(|a| u64::try_from(a).ok()) {
        Some(a) => match whole.over([a, a, 1, 1]) {
            Some(share) => share,
            None => whole,
        },
        None => whole,
    }
}

impl Transduction {
    /// THE RECORD OF A PAIR under the caller's stroke, sized by
    /// [`need_of`]: one code path at every rank. `Rank` where the two
    /// samples differ in rank; `Spent { need }` where the stroke ends first,
    /// with no record returned.
    pub fn of(x: &Sample, y: &Sample, budget: &mut crate::physics::Budget) -> Result<Transduction, TransductionRefusal> {
        let from = Enclosed::of(x);
        let to = Enclosed::of(y);
        let (p, q) = (from.enclosure.periods().to_vec(), to.enclosure.periods().to_vec());
        if p.len() != q.len() {
            return Err(TransductionRefusal::Rank { from: p.len(), to: q.len() });
        }
        let need = need(&from.enclosure, &to.enclosure);
        let (&[h, ..], &[h2, ..]) = (p.as_slice(), q.as_slice()) else { return Err(TransductionRefusal::Empty) };
        // The output's bands by the slice arcs of the first phase they span,
        // each slice in the canonical order of the bands' spans past it.
        let mut slices: Vec<Vec<Located>> = vec![Vec::new(); h2];
        for (i, b) in to.bands.iter().enumerate() {
            let [(lo, hi), past @ ..] = b.spans.as_slice() else { continue };
            for r in *lo..*hi {
                if let Some(slice) = slices.get_mut(r) {
                    slice.push((past.to_vec(), i));
                }
            }
        }
        for slice in &mut slices {
            slice.sort_unstable();
        }
        let mut incidences = Vec::new();
        for (a, band) in from.bands.iter().enumerate() {
            let &[(r0, r1), ..] = band.spans.as_slice() else { continue };
            // The output slice arcs that meet the band's first arc with
            // positive length: `⌊r0·h′/h⌋ ≤ r′ < ⌈r1·h′/h⌉`. Both bounds are
            // at most `h′`, so they fit where `h′` does.
            let (Ok(lo), Ok(hi)) = (
                usize::try_from(r0 as u128 * h2 as u128 / h as u128),
                usize::try_from((r1 as u128 * h2 as u128).div_ceil(h as u128)),
            ) else {
                continue;
            };
            for r2 in lo..hi {
                let Some(slice) = slices.get(r2) else { continue };
                for &(_, b) in slice {
                    let Some(out) = to.bands.get(b) else { continue };
                    // The slice is in order of the second phase's first
                    // turn: past the band's last turn there, none meets it.
                    if let (Some(&(c0, _)), Some(&(_, d1)), Some(&w2), Some(&w)) =
                        (out.spans.get(1), band.spans.get(1), q.get(1), p.get(1))
                    {
                        if Turn::of(c0, w2) >= Turn::of(d1, w) {
                            break;
                        }
                    }
                    let every = (1..p.len()).all(|k| match (band.spans.get(k), p.get(k), out.spans.get(k), q.get(k)) {
                        (Some(&s), Some(&pk), Some(&t), Some(&qk)) => meets(s, pk, t, qk),
                        _ => false,
                    });
                    if !every {
                        continue;
                    }
                    if !budget.spend() {
                        return Err(TransductionRefusal::Spent { need });
                    }
                    // One incidence per band pair, at the first slice arc the
                    // two share.
                    let &[(o0, _), ..] = out.spans.as_slice() else { continue };
                    if r2 != lo.max(o0) {
                        continue;
                    }
                    incidences.push(Incidence { band_in: a, band_out: b });
                }
            }
        }
        Ok(Transduction { from, to, incidences })
    }

    /// Does the re-period keep every period: both frames of one enclosure?
    #[cfg(test)]
    pub fn keeps_periods(&self) -> bool {
        self.from.enclosure.is_same_periods(&self.to.enclosure)
    }

    /// The two bands of an incidence.
    fn bands_of(&self, i: &Incidence) -> Option<(&Band, &Band)> {
        Some((self.from.bands.get(i.band_in)?, self.to.bands.get(i.band_out)?))
    }

    /// The incidences whose symbols differ: the change the pair records.
    pub fn changed(&self) -> impl Iterator<Item = &Incidence> {
        self.incidences.iter().filter(|i| self.bands_of(i).is_some_and(|(a, b)| a.symbol() != b.symbol()))
    }

    /// O1 — THE SHARE OF THE FRAME THE PAIR CHANGED, exact: the sum over
    /// its changed incidences of the product, over the `n` phases, of the
    /// overlap's length in turns: the flat measure of the torus face (a
    /// product of the circles' measures, never a square). Resolution-free;
    /// on one enclosure it is the changed sites over the frame's measure.
    pub fn share(&self) -> crate::core::Q {
        self.changed().filter_map(|i| self.measure_of(i)).fold(crate::core::Q::from_integer(0.into()), |s, m| s + m)
    }

    /// The flat measure of one incidence's overlap, phase by phase.
    fn measure_of(&self, i: &Incidence) -> Option<crate::core::Q> {
        let (a, b) = self.bands_of(i)?;
        let (p, q) = (self.from.enclosure.periods(), self.to.enclosure.periods());
        let mut m = crate::core::Q::from_integer(1.into());
        for k in 0..p.len() {
            let (t0, t1) = overlap(*a.spans.get(k)?, *p.get(k)?, *b.spans.get(k)?, *q.get(k)?);
            m *= t1.value() - t0.value();
        }
        Some(m)
    }

    /// O1 (ruled 2026-11-01) — THE SHARE BY LANE, `[S, G]`: each changed
    /// incidence's flat measure lands on S where exactly one of its two
    /// symbols is read as ground (a site changed between ground and
    /// painted) and on G where neither or both are (one painted symbol to
    /// another, or one candidate of a vortex to another). The ground is the
    /// family's deduced reading, handed in (MU-SENSING §3.4): under
    /// `Ground::None` every change lands on G. The two sum to
    /// [`Transduction::share`]; on one enclosure they are the coboundary's
    /// spatial and gauge counts over the frame's measure.
    pub fn lane_shares(&self, ground: &super::Ground<Vec<u8>>) -> [crate::core::Q; 2] {
        let mut lanes = [crate::core::Q::from_integer(0.into()), crate::core::Q::from_integer(0.into())];
        for i in self.changed() {
            let (Some((a, b)), Some(m)) = (self.bands_of(i), self.measure_of(i)) else { continue };
            let lane = match ground.holds_symbol(a.symbol()) != ground.holds_symbol(b.symbol()) {
                true => 0,
                false => 1,
            };
            lanes[lane] += m;
        }
        lanes
    }

    /// PROJECT (D3.2 §E) — THE RECORD READ BACK ON THE OUTPUT FRAME: what
    /// the input, re-periodded and nothing else, says against the output,
    /// site by site: the residue of the identity's speech, at every output
    /// site whose symbols differ, in canonical address order. Every band
    /// boundary turn of the input lands on an output boundary or the
    /// projection refuses `OffSite` naming it: a boundary inside an output
    /// compartment would speak two symbols there, and none is chosen. On
    /// one enclosure it is the coboundary link for link.
    pub fn projected(&self) -> Result<Vec<SiteChange>, TransductionRefusal> {
        let q = self.to.enclosure.periods();
        let mut landed: Vec<Vec<(usize, usize)>> = Vec::with_capacity(self.from.bands.len());
        for b in &self.from.bands {
            let mut spans = Vec::with_capacity(q.len());
            for ((t0, t1), &qk) in b.turns(&self.from.enclosure).iter().zip(q) {
                spans.push((t0.lands(qk)?, t1.lands(qk)?));
            }
            landed.push(spans);
        }
        let mut out = Vec::new();
        for i in self.changed() {
            let (Some(spans), Some((a, b))) = (landed.get(i.band_in), self.bands_of(i)) else { continue };
            let meet: Vec<(usize, usize)> =
                spans.iter().zip(&b.spans).map(|(&(s0, s1), &(t0, t1))| (s0.max(t0), s1.min(t1))).collect();
            let mut sites: Vec<Vec<i64>> = vec![Vec::new()];
            for &(lo, hi) in &meet {
                sites = sites
                    .into_iter()
                    .flat_map(|s| {
                        (lo..hi).filter_map(move |c| {
                            let mut next = s.clone();
                            next.push(i64::try_from(c).ok()?);
                            Some(next)
                        })
                    })
                    .collect();
            }
            for axes in sites {
                let Some(site) = Site::new(axes) else { continue };
                out.push(SiteChange { site, before: a.symbol().bytes().to_vec(), after: b.symbol().bytes().to_vec() });
            }
        }
        // Canonical address order: the sites are distinct (each output site
        // lies in one output band and one landed input band), so the order
        // is the sites' alone and no symbol is ordered.
        let mut order: Vec<(Site, usize)> = out.iter().enumerate().map(|(k, c)| (c.site.clone(), k)).collect();
        order.sort_unstable();
        let mut held: Vec<Option<SiteChange>> = out.into_iter().map(Some).collect();
        Ok(order.into_iter().filter_map(|(_, k)| held.get_mut(k).and_then(Option::take)).collect())
    }

    /// THE OUTPUT BODY AN INPUT BODY IS CARRIED ONTO under the re-period:
    /// the one output body its bands meet, where that body meets no other
    /// input body. The frames' bands partition both frames, so then the
    /// body's image IS that body, boundary turn for boundary turn. Any other
    /// meeting (split across bodies, or merged with another) is `Mixed`,
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
