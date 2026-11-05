// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! THE RASTER AT INTAKE AND THE SECTION'S READINGS (MU-SENSING §2; ROADMAP
//! track 26, S1, S4d): the bridge that hands a source's lines to the sensor
//! as a [`Sample`], and the readings the word machinery takes of a sample on
//! the alphabet's section ([`SectionSample`]): its extents, its ground, its
//! painted sites and its frame fixed where the frame is a gauge. A source's
//! decoder, kept outside `src/` (MU-SENSING §7), writes the lines; nothing
//! here reads a file format or names a source.
//!
//! The ground is deduced per family ([`ground_of`], track 26 S2) over the
//! samples themselves, each at its own rank, and never defaults to a symbol.
//! The planar readings are the section's, of the rank the letters spell
//! (`alphabet::ALPHABET_SAMPLE_RANK`); no carrier here fixes a rank of its
//! own (track 26 S4d).
//!
//! No module of the core is named for a data source:
//!
//! ```compile_fail,E0432
//! use mycelium::arc;
//! ```

use super::family::transduce::{Ground, Unground};
use super::family::{Family, Task};
use super::alphabet::ALPHABET_SAMPLE_RANK;
use super::sample::{Place, Sample, SampleRefusal};
use super::spelled::{PaintedSite, SectionSample, SectionSite};

/// THE WIDTH OF A RASTER. A raster is rectangular (every reader that takes
/// one refuses a ragged one), so line zero is a COORDINATE of the raster
/// and its length is the raster's width, not the length of whichever line
/// arrived first (rule 15). A raster with no line has no sites and
/// measures zero.
pub fn raster_width(g: &[Vec<u8>]) -> usize {
    #[expect(
        clippy::disallowed_methods,
        reason = "rule 15 — canonical key order: line zero is the raster's own coordinate, and a rectangular raster's width is its length"
    )]
    g.first().map_or(0, Vec::len)
}

/// THE SCENE'S AMBIENT GROUND Ω, DEDUCED (MU-SENSING §3.4; track 26 S2):
/// the symbol the bodies of a family stand on, read over its samples by
/// [`Ground::of`] (ambient, enclosing, never carried as a body), each
/// candidate held as the section's one byte. Never a code: no symbol is
/// the ground by default, by count or by order. The symbols are never relabelled
/// (78.56 II): a ground of 7 stays 7 on every raster, word and wire.
pub fn ground_of(family: &Family) -> Ground<u8> {
    let pairs: Vec<(Sample, Sample)> = family.iter().map(|(x, y)| (x.sample().clone(), y.sample().clone())).collect();
    Ground::of(&pairs).map(|s| match s.as_slice() {
        &[b] => Some(b),
        _ => None,
    })
}

thread_local! {
    /// The ground of the family being read on this thread; `None` where no
    /// family's reading is open, and then no ground is read at all.
    static GROUND: std::cell::RefCell<Option<Ground<u8>>> = const { std::cell::RefCell::new(None) };
}

/// THE GROUND OF THE SCENE BEING READ: the open family's reading. Where no
/// family is open no ground is read, so the reading is `Ground::None` and
/// every symbol is a body: an unscoped reader never stands on a code
/// (MU-SENSING §3.4), and a writer that needs a symbol is refused
/// [`Unground::Unscoped`] by [`void`].
pub fn ground() -> Ground<u8> {
    GROUND.with(|c| match &*c.borrow() {
        Some(g) => g.clone(),
        None => Ground::None,
    })
}

/// IS `h` READ AS GROUND in the open scope ([`Ground::holds`]); never where
/// no family is open.
pub fn is_ground(h: u8) -> bool {
    GROUND.with(|c| c.borrow().as_ref().is_some_and(|g| g.holds(&h)))
}

/// THE ONE SYMBOL A RASTER WRITER WRITES FOR THE GROUND, or the refusal that
/// names why there is none: `Vortex` (no candidate is picked), `None` (the
/// family reads no ground), `Unscoped` (no family is open).
pub fn void() -> Result<u8, Unground> {
    GROUND.with(|c| match &*c.borrow() {
        Some(g) => g.symbol().copied(),
        None => Err(Unground::Unscoped),
    })
}

/// READ `f` ON THE GROUND OF `family`: the family's ground opened for every
/// perception, speech and replay inside it, UNLESS a family is already open
/// on this thread, whose ground stands (the siblings a perspective derives
/// from, a test spoken through the family's rule, are read on the family's
/// own ground, never on one a subset of the siblings would name). The scope
/// closes as `f` returns.
pub fn on_ground_of<T>(family: &Family, f: impl FnOnce() -> T) -> T {
    match GROUND.with(|c| c.borrow().is_some()) {
        true => f(),
        false => on_ground(ground_of(family), f),
    }
}

/// READ `f` ONCE PER PERSPECTIVE OF `family`'s GROUND (MU-SENSING §3.8,
/// §4; track 26 S2): under `Ground::Vortex` once per candidate, each run on
/// `Ground::One` of it; under `One` or `None` once, on it. Every result is
/// kept with the perspective it was read in, in the vortex's order: the
/// vortex's sections, none picked. Where a family's reading is already
/// open, its perspectives are read instead.
pub fn on_each_perspective_of<T>(family: &Family, mut f: impl FnMut() -> T) -> Vec<(Ground<u8>, T)> {
    let open = GROUND.with(|c| c.borrow().clone());
    let reading = match open {
        Some(g) => g,
        None => ground_of(family),
    };
    reading.perspectives().into_iter().map(|p| (p.clone(), on_ground(p, &mut f))).collect()
}

/// THE GROUND A CLAIM STATES, spoken under perspective `ground`: its one
/// symbol under `Ground::One`, none otherwise (a raster's symbol at width
/// one). What the solve act carries (`dialogue::encode_solve_spoken`).
pub fn stated(ground: &Ground<u8>) -> Option<Vec<u8>> {
    match ground {
        Ground::One(g) => Some(vec![*g]),
        Ground::Vortex(_) | Ground::None => None,
    }
}

/// READ `f` ON GROUND `g`, whatever was open; the previous scope restored as
/// `f` returns.
pub fn on_ground<T>(g: Ground<u8>, f: impl FnOnce() -> T) -> T {
    let before = GROUND.with(|c| c.replace(Some(g)));
    let out = f();
    GROUND.with(|c| *c.borrow_mut() = before);
    out
}

/// A TEST FIXTURE'S DECLARED GROUND: the fixture's family is written on the
/// one symbol `g`, and every reader on this thread stands on it until the
/// thread ends (each test runs on a thread of its own). A declaration of
/// the fixture, never a default: a reader outside it reads no ground. A
/// ground the test already opened stands.
#[cfg(test)]
pub(crate) fn stand_on(g: u8) {
    GROUND.with(|c| {
        let mut open = c.borrow_mut();
        if open.is_none() {
            *open = Some(Ground::One(g));
        }
    });
}

/// A TASK'S FRAME FIXED ON THE FAMILY'S OWN GROUND: under `Ground::One(g)`
/// the moments of [`frame_fixed`] weigh the sites off `g` the train family
/// shows. Under a vortex or no ground the frame stays as read: no candidate
/// is picked to weigh by, and the frame becomes a perspective in S6
/// (MU-SENSING §4). A perspective a sensor may take after intake, never
/// before a task is addressed: no decoder hands its tasks through it, and
/// a task's identity reads no frame fixing (MU-SENSING §3.4, scope
/// separation).
pub fn frame_fixed_on_ground(task: Task) -> Task {
    // The perspective open on this thread stands, as `on_ground_of` keeps
    // it; with none open, the train family's deduced reading.
    let open = GROUND.with(|c| c.borrow().clone());
    let reading = match open {
        Some(g) => g,
        None => ground_of(&task.train),
    };
    match reading {
        Ground::One(g) => on_ground(Ground::One(g), || frame_fixed(task)),
        Ground::Vortex(_) | Ground::None => task,
    }
}

/// 78.54, 78.56 — THE LATTICE'S FRAME IS A GAUGE WHERE THE TASK CARRIES NO
/// POLAR VECTOR. A task turned by 90°, 180° or 270°, or mirrored, as a
/// whole (every raster of it) is the same task when its centre of mass does
/// not move from input to output (Δr_CM = 0, the O(2) singlet): the frame
/// the sensor happened to read it in is a coordinate choice, and a word that
/// carried it would make a mind that learned a motion in one frame blind to
/// it in another. Where Δr_CM ≠ 0 the orientation is physical and kept.
///
/// The frame is fixed at the sensor and nowhere after:
/// READ, AXIS BY AXIS, OFF THE SCENE'S OWN MOMENTS, never by enumerating the
/// lattice's symmetry group (2ⁿ·n! poses in n axes; rules 3 and 4). Over the
/// VISIBLE rasters (train pairs and test inputs; a withheld output never
/// moves the frame), each painted site a unit of mass, in centred
/// coordinates:
/// - an axis is REVERSED where its first non-zero odd moment (orders 1, 3)
///   is negative, which is what a reflection along it changes;
/// - the axes are ORDERED by the axes' even moments (orders 2, 4), which is
///   what a permutation of the axes changes.
///
/// These are a gauge-fixing condition, the representative the geometry
/// itself names, never a candidate chosen by a scalar. They read no symbol.
/// Where an axis's odd moments all vanish, or two axes' even moments tie,
/// the scene is symmetric under that move as far as these orders read, and
/// the frame given is kept (law 10: held, never broken by an arbitrary
/// choice). The symbols are never relabelled: the detector is a Dirichlet
/// boundary, the laboratory's anchor for every mind (78.56 II).
pub fn frame_fixed(task: Task) -> Task {
    // 78.56 — A TASK WITH A POLAR VECTOR HAS A PHYSICAL ORIENTATION: its
    // momentum breaks the lattice's symmetry, and its frame is kept.
    if !momentum_vanishes(&task.train) {
        return task;
    }
    // THE MOMENTS, per axis and per order 1..=4, over the visible samples.
    let mut moment = [[0i128; 4]; ALPHABET_SAMPLE_RANK];
    let visible = task.train.iter().flat_map(|(a, b)| [a, b]).chain(task.test.iter().map(|(a, _)| a));
    for g in visible {
        let e = g.extents();
        for p in painted_sites(g) {
            let site = p.site.coordinates();
            for (axis, m) in moment.iter_mut().enumerate() {
                let x = centred(site[axis], e[axis]);
                let mut power = 1i128;
                for order in m.iter_mut() {
                    power *= x;
                    *order += power;
                }
            }
        }
    }
    let reversed: [bool; ALPHABET_SAMPLE_RANK] = std::array::from_fn(|axis| {
        let [first, _, third, _] = moment[axis];
        match first {
            0 => third < 0,
            f => f < 0,
        }
    });
    let mut order: [usize; ALPHABET_SAMPLE_RANK] = std::array::from_fn(|axis| axis);
    // A stable ordering: axes whose even moments tie keep the order given.
    #[expect(clippy::disallowed_methods, reason = "rule 4 — a gauge-fixing condition, not a choice among candidates: the axes are ordered by the even moments the scene itself carries (78.54)")]
    order.sort_by_key(|&axis| (moment[axis][1], moment[axis][3]));
    let at = |g: &SectionSample| reoriented(g, order, reversed);
    Task {
        train: task.train.iter().map(|(a, b)| (at(a), at(b))).collect(),
        test: task.test.iter().map(|(a, b)| (at(a), b.as_ref().map(at))).collect(),
    }
}

/// 78.56 — WHETHER THE TASK CARRIES NO POLAR VECTOR: its momentum (the
/// mass-weighted displacement from input to output, `Σ_out X − s·Σ_in X`
/// in centred coordinates across each pair's envelope dilation, summed
/// over the train pairs, every painted site a unit of mass) is exactly
/// zero. `false` where it is not, and where any pair's dilation is
/// irrational (its momentum unread: the orientation is held physical, so
/// the frame is never un-fixed by a guess). Read by [`frame_fixed`] alone.
pub fn momentum_vanishes(train: &Family) -> bool {
    use num_rational::Ratio;
    let mut total: [Ratio<i128>; ALPHABET_SAMPLE_RANK] = std::array::from_fn(|_| Ratio::from_integer(0));
    for (a, b) in train {
        let Some((p, q)) = super::family::envelope_dilation(a.sample(), b.sample()) else { return false };
        // A scene's first moment, per axis, each painted site a unit of mass.
        let moment = |g: &SectionSample| -> [i128; ALPHABET_SAMPLE_RANK] {
            let e = g.extents();
            let mut m = [0i128; ALPHABET_SAMPLE_RANK];
            for p in painted_sites(g) {
                let site = p.site.coordinates();
                for (axis, at) in m.iter_mut().enumerate() {
                    *at += centred(site[axis], e[axis]);
                }
            }
            m
        };
        let (m_in, m_out) = (moment(a), moment(b));
        for (axis, t) in total.iter_mut().enumerate() {
            *t += Ratio::from_integer(m_out[axis]) - Ratio::new(m_in[axis] * i128::from(p), i128::from(q));
        }
    }
    total.iter().all(|t| *t == Ratio::from_integer(0))
}

/// THE SENSOR'S ONE READING OF A SECTION SAMPLE: each painted site (the
/// sample's coordinate vector restricted to the perspective's plane) with
/// its symbol, in canonical address order. The one pass over a scene's
/// sites (law 11's sensor clause); every reader here takes it. Every
/// geometric reader here is written once per axis of the plane, never an
/// axis by hand.
///
/// The PERCEPTUAL convention: the scene's ground is omitted (the eye sees
/// only novelty).
pub fn painted_sites(g: &SectionSample) -> Vec<PaintedSite> {
    let ground = ground();
    g.sites().filter(|(_, h)| !ground.holds(h)).map(|(site, h)| PaintedSite::new(site, h)).collect()
}

/// A coordinate centred on its extent and doubled, so every centre is an
/// integer: `X = 2x − (n − 1)`.
pub(crate) fn centred(x: i64, n: usize) -> i128 {
    2 * i128::from(x) - (n as i128 - 1)
}

/// A section sample with the axes of its plane permuted (`order[j]` is the
/// axis read as the j-th) and reversed where `reversed[axis]`: one pass over
/// every site, so no symbol is written that the sample did not hold. The
/// sample keeps its place.
fn reoriented(g: &SectionSample, order: [usize; ALPHABET_SAMPLE_RANK], reversed: [bool; ALPHABET_SAMPLE_RANK]) -> SectionSample {
    match reoriented_of(g, order, reversed) {
        Some(s) => s,
        None => g.clone(),
    }
}

/// [`reoriented`], or `None` where the permuted extents hold no sample on
/// the section or a site reads none of `g`'s.
fn reoriented_of(g: &SectionSample, order: [usize; ALPHABET_SAMPLE_RANK], reversed: [bool; ALPHABET_SAMPLE_RANK]) -> Option<SectionSample> {
    let e = g.extents();
    let ne: [usize; ALPHABET_SAMPLE_RANK] = std::array::from_fn(|j| e[order[j]]);
    // The permuted extents hold the same count of sites: the canvas holds
    // `g`'s symbols until every site of it is written below, each once.
    let sample = Sample::new(ne.to_vec(), 1, g.symbols().to_vec(), g.sample().place().clone()).ok()?;
    let mut out = SectionSample::of(sample).ok()?;
    let mut writes = Vec::with_capacity(g.symbols().len());
    for (at, _) in out.sites() {
        let at = at.coordinates();
        // The source site: output axis `i` reads source axis `order[i]`.
        let mut site = [0i64; ALPHABET_SAMPLE_RANK];
        for (i, &axis) in order.iter().enumerate() {
            site[axis] = match reversed[axis] {
                true => i64::try_from(e[axis]).ok()? - 1 - at[i],
                false => at[i],
            };
        }
        writes.push((SectionSite::on_plane(at), g.at(SectionSite::on_plane(site))?));
    }
    for (at, h) in writes {
        out.paint(at, h)?;
    }
    Some(out)
}

// ── THE SAMPLE BRIDGE (ROADMAP track 26, S4; MU-SENSING §2) ────────────────
//
// A source's lines reach the sensor only as a `Sample` read here; past
// intake the word machinery reads samples on the alphabet's section
// (`spelled::SectionSample`), and no transduce file reads a raster.

/// A RASTER AS A SAMPLE: rank 2, extents (axis 0, axis 1), one byte per
/// site in canonical address order (axis 0 slowest). A ragged raster is not
/// the extent it claims and is refused `Length`; a raster with no line or
/// an empty line is refused `EmptyAxis` by the sample.
pub fn sample_of(g: &[Vec<u8>], place: Place) -> Result<Sample, SampleRefusal> {
    let width = raster_width(g);
    if g.iter().any(|line| line.len() != width) {
        let expected = g.len().checked_mul(width).ok_or(SampleRefusal::SitesOverflow { axis: 1 })?;
        return Err(SampleRefusal::Length { expected, found: g.iter().map(Vec::len).sum() });
    }
    Sample::new(vec![g.len(), width], 1, g.concat(), place)
}

/// A PROJECTED RECORD AS THE SECTION'S CHAIN: the speech end of the bridge
/// for a residue, each change read as a link of the section
/// (`sentence::Link::section`). A change off the section's rank or wider
/// than one byte has no link there and is left out, which no raster pair
/// produces.
pub fn links_of(changes: &[super::family::transduce::SiteChange]) -> Vec<crate::transducer::sentence::SectionLink> {
    changes
        .iter()
        .filter_map(|c| crate::transducer::sentence::Link::of_change(c).section().ok())
        .collect()
}

/// THE GROUND AS THE SYMBOLS A RASTER'S SAMPLE WRITES: each candidate of a
/// raster reading as its one byte. The transduction takes the ground as a
/// reading handed in, never as a code of its own.
pub fn ground_bytes(ground: &Ground<u8>) -> Ground<Vec<u8>> {
    ground.map(|&b| Some(vec![b]))
}

/// [`sample_of`] for a reader that holds no family coordinates for the
/// raster: the sample's place is empty, a place the family has not written,
/// never a coordinate invented for it.
pub fn sample_unplaced(g: &[Vec<u8>]) -> Result<Sample, SampleRefusal> {
    sample_of(g, Place::new(Vec::new()))
}
