// 53.1, 53.2 — THE SCENE COVER: the regions a scene is cut into, under a
// reading, before it is perceived as one form.
//
//! A scene arriving at extraction as one form of 712 vertices is not a
//! cost to optimise but the absence of a cut (53). This organ is the cut:
//! a **reading** is a way the sensor plane is divided, a **region** is a
//! part of it named by its support written out, and a **cover** is the
//! regions of one reading, keyed.
//!
//! THE ROSTER IS THE ENUM. `PlaneCut` has one member today and a new one
//! enters behind a court receipt on a probe (law 6 as amended, rows K4 and
//! K5) — never by a voice and never by a commit that merely adds a variant
//! because it seemed useful. The enum IS the roster, so the governance has
//! somewhere to bind.
//!
//! NO READING IS CHOSEN (rules 4, 15). `ROSTER` is walked in its own
//! order, every cover that closes is kept, and the caller holds
//! incomparable ones rather than taking the first that answers.

use super::perceive::{frame_of, SectionSample, SectionSite};
use crate::network::nerve::Keyed;
use crate::transducer::base::Site;
use crate::transducer::layers::LayerKind;

/// A WAY THE SENSOR PLANE IS CUT (53.2). The roster is fixed and grows
/// only by a court receipt on a probe — a reading whose first cover on the
/// corpus closes a family no other reading closes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlaneCut {
    /// The uniform separator rows and columns every input of the family
    /// carries. Deducible in one pass over the inputs, and the cut 850 of
    /// the 1 120 corpus tasks present.
    Bands,
}

impl PlaneCut {
    /// THE ROSTER, in its own order. Walked whole by the caller; a reading
    /// is never picked out of it by a measurement (rule 4).
    pub const ROSTER: [PlaneCut; 1] = [PlaneCut::Bands];

    /// The reading's own byte, ahead of every region key it names, so two
    /// readings that cut a scene into the same region name two regions and
    /// a `Keyed` of regions under one reading is a stratum of its tag.
    pub fn byte(self) -> u8 {
        match self {
            PlaneCut::Bands => 0,
        }
    }
}

/// A REGION OF THE SENSOR PLANE, by its support on the sites of the base
/// (51.3, 71). A region is itself a scene and is covered again; what ends
/// the recursion is 53.1's identity of coordinates, below.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Region {
    /// The sites the region holds, in coordinate order.
    pub support: Vec<Site>,
}

/// THE REGIONS OF ONE READING, each keyed by its own coordinates.
pub type Regions = Keyed<Region>;

/// A REGION'S KEY: the reading's tag, then the region written out as the
/// [`coord::Piece`] it is (A21, §4.4 E8), of which every prefix is a
/// coarser stratum.
///
/// [`coord::Piece`]: crate::network::nerve::coord::Piece
///
/// `LayerKind::Frame` is the kind because a region is a part of the
/// GRID'S OWN EXTENT and not a body layer — which is what that member of
/// the enum says. A reading that cut by something beyond the region would
/// write it after its own tag, which leads the key, so its regions key apart
/// from these without a second key family (rule 6).
pub fn region_key(reading: PlaneCut, support: &[Site]) -> Vec<u8> {
    let mut k = vec![reading.byte()];
    use crate::network::nerve::Coordinate;
    k.extend_from_slice(super::pieces::piece_at(LayerKind::Frame, support).encode().as_bytes());
    k
}

impl Region {
    pub fn key(&self, reading: PlaneCut) -> Vec<u8> {
        region_key(reading, &self.support)
    }
}

/// THE WHOLE SCENE AS ONE REGION — the coordinate the recursion's end is
/// compared against. `None` where the scene has no frame to name.
pub fn scene_region(scene: &[SectionSample]) -> Option<Region> {
    let [first, rest @ ..] = scene else { return None };
    let (h, w) = frame_of(first);
    if h == 0 || w == 0 {
        return None;
    }
    // ONE FRAME OR NO READING. A cover of regions cut from two different
    // extents names sites that are not the same sites (rule 11).
    for g in rest {
        if frame_of(g) != (h, w) {
            return None;
        }
    }
    Some(Region { support: block(0, h as i64, 0, w as i64) })
}

/// THE SITES OF A RECTANGULAR BLOCK, in coordinate order. Frame `0`: a
/// region is a part of the sensor plane and the frame rides on the pose
/// (16.5), so the cut is one reading for every frame of the family.
fn block(r0: i64, r1: i64, c0: i64, c1: i64) -> Vec<Site> {
    let mut out = Vec::new();
    // index: the block's own sites, which are what it IS
    for r in r0..r1 {
        for c in c0..c1 {
            out.push(Site::from((r, c, 0)));
        }
    }
    out
}

/// THE COVER'S OWN KEY: its regions' keys written out length-delimited, in
/// the trie's order, which is the regions' coordinate order.
///
/// It exists so 53.1's identity end is A KEY COMPARISON and not a count. A
/// cover of one region equal to the whole scene has the same key as the
/// one-element listing of the scene's own region, and no other cover does:
/// a second region lengthens the listing, and a single region cut smaller
/// writes different coordinates. `len() == 1`, `<`, and any constant are
/// refused here under rules 2 and 5.
pub fn cover_key(cover: &Regions) -> Vec<u8> {
    let mut k = Vec::new();
    for (ck, _) in cover.scoped() {
        crate::transducer::address::put_key_bytes(&mut k, &ck);
    }
    k
}

/// THE RECURSION'S END (53.1): the cover's key IS the scene's own key.
///
/// A region is itself a scene and is covered again; this is where that
/// stops. There is no floor and no count — `FRAGMENT_FLOOR = 9` is refused
/// by rule 5 and is not replaced by a count of regions (rule 2).
pub fn is_identity(cover: &Regions, scene: &[SectionSample], reading: PlaneCut) -> bool {
    let Some(whole) = scene_region(scene) else {
        return false;
    };
    let mut want = Vec::new();
    crate::transducer::address::put_key_bytes(&mut want, &whole.key(reading));
    cover_key(cover) == want
}

/// THE COVER A READING DEDUCES (53.1). `None` is "this reading deduces no
/// cover on this scene", which is not a demand and not a refusal: the
/// caller walks the rest of the roster.
pub fn cover(scene: &[SectionSample], reading: PlaneCut) -> Option<Regions> {
    match reading {
        PlaneCut::Bands => bands(scene),
    }
}

/// Is the run of sites at `coordinate` on the plane's axis `axis` one hue
/// throughout? A coordinate the sample does not reach holds no run.
fn uniform_at(g: &SectionSample, axis: usize, coordinate: usize) -> bool {
    let Ok(coordinate) = i64::try_from(coordinate) else { return false };
    let mut run = g.sites().filter(|(s, _)| s.coordinates().get(axis) == Some(&coordinate)).map(|(_, h)| h);
    let Some(want) = run.next() else { return false };
    run.all(|h| h == want)
}

/// Is this row of this grid one hue throughout?
fn row_uniform(g: &SectionSample, r: usize) -> bool {
    uniform_at(g, 0, r)
}

/// Is this column of this grid one hue throughout?
fn col_uniform(g: &SectionSample, c: usize) -> bool {
    uniform_at(g, 1, c)
}

/// THE MAXIMAL RUNS OF NON-SEPARATOR INDICES, in coordinate order: the
/// bands the separators leave. A run's two ends are its coordinates and
/// nothing is chosen between them.
fn runs(sep: &[bool]) -> Vec<(i64, i64)> {
    let mut out: Vec<(i64, i64)> = Vec::new();
    let mut open: Option<i64> = None;
    // index: the scene's own extent along one axis, which is its boundary
    for (i, &is_sep) in sep.iter().enumerate() {
        match (is_sep, open) {
            (true, Some(start)) => {
                out.push((start, i as i64));
                open = None;
            }
            (true, None) => {}
            (false, None) => open = Some(i as i64),
            (false, Some(_)) => {}
        }
    }
    if let Some(start) = open {
        out.push((start, sep.len() as i64));
    }
    out
}

/// 53.2 BANDS: the rows and columns uniform across EVERY input of the
/// family bound the regions.
///
/// A row is a separator where it is one hue throughout in every grid of
/// the scene — the hue may differ from grid to grid, because what the
/// reading deduces is that nothing of the scene's content crosses there.
/// The regions are the maximal runs of non-separator rows crossed with the
/// maximal runs of non-separator columns.
///
/// A scene with no separator is its own single region, which is 53.1's
/// identity end reached in one pass and NOT a special case written here:
/// `runs` over a boundary with no separator in it is the one run that
/// spans it, and the block that names is the whole scene.
fn bands(scene: &[SectionSample]) -> Option<Regions> {
    let whole = scene_region(scene)?;
    let [first, ..] = scene else { return None };
    let (h, w) = frame_of(first);
    let rows: Vec<bool> = (0..h).map(|r| scene.iter().all(|g| row_uniform(g, r))).collect();
    let cols: Vec<bool> = (0..w).map(|c| scene.iter().all(|g| col_uniform(g, c))).collect();
    let (rr, cc) = (runs(&rows), runs(&cols));
    let mut out: Regions = Keyed::new();
    for &(r0, r1) in &rr {
        for &(c0, c1) in &cc {
            let region = Region { support: block(r0, r1, c0, c1) };
            out.insert(region.key(PlaneCut::Bands), region);
        }
    }
    // A SCENE THAT IS ALL SEPARATOR HAS NO BAND, and an empty cover is not
    // a cover: the reading deduced nothing here (rule 11). The `whole` is
    // read above so a scene with no frame refuses before any of this.
    if out.is_empty() {
        let _ = whole;
        return None;
    }
    Some(out)
}

/// A SAMPLE RESTRICTED TO A REGION: the region's own sites, read at the
/// region's origin, so a region's family is an ordinary family of its own
/// extent and the moves below it are 52.5's unchanged.
///
/// `None` where the region names a site the sample does not have — a region of
/// one scene restricted against another's extent, which is not a reading
/// of anything (rule 11).
pub fn restrict(g: &SectionSample, region: &Region) -> Option<SectionSample> {
    // The region's section sites: a region is a part of the sensor plane
    // (16.5).
    let plane: Vec<SectionSite> = region.support.iter().map(SectionSite::of_family).collect::<Option<_>>()?;
    // THE REGION'S EXTENT, its least corner and the extents from it, read
    // off every site (`SectionSite::span_of`): every offset below is from the
    // true least corner, so it is exact in `usize` or the restriction is
    // refused (rule 11), never wrapped; no site is no region.
    let (corner, extents) = SectionSite::span_of(plane.iter().copied())?;
    let mut out = SectionSample::filled(extents, crate::transducer::raster::void().ok()?).ok()?;
    for &site in &plane {
        let hue = g.at(site)?;
        out.paint(site.offset_from(corner)?, hue)?;
    }
    Some(out)
}
