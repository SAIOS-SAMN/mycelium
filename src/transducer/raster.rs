// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! THE RANK-2 RASTER AT INTAKE (MU-SENSING §2; ROADMAP track 26, S1): the
//! two-level carrier `Vec<Vec<u8>>` the transduction still reads, its
//! extents, its ground, its painted sites, its frame fixed where the frame
//! is a gauge, and the bridge that hands it to the sensor as a [`Sample`]
//! and takes a spoken sample back. A source's decoder, kept outside `src/`
//! (MU-SENSING §7), writes the raster; nothing here reads a file format or
//! names a source.
//!
//! Two defects stand here unchanged and named (MU-SENSING §8): the carrier
//! is rank 2 by its own storage ([`AXES`], track 26 S4), and the ground is
//! the symbol `0` where no family is open ([`ground`], track 26 S2).
//!
//! No module of the core is named for a data source:
//!
//! ```compile_fail,E0432
//! use mycelium::arc;
//! ```

use super::family::{Family, Task};
use super::sample::{AxisReading, Place, Sample, SampleRefusal};

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

/// THE SCENE'S AMBIENT GROUND Ω (ruled 2026-10-18): the symbol the bodies
/// of a family stand on, the void the eye does not see. A family that
/// shows the symbol 0 on any raster of its demonstration pairs stands on 0.
/// A family that never shows it is drawn on a canvas: its ground is the
/// symbol of the greatest site count over those rasters, an exact count in
/// ℤ; where two symbols tie for it the scene names no ground of its own and
/// the void is 0 (`Tie::Hold`, law 10). The symbols are never relabelled
/// (78.56 II): a ground of 7 stays 7 on every raster, word and wire; Ω only
/// says which symbol the eye reads as the void. The default 0 is the defect
/// MU-SENSING §3.4 removes (track 26, S2).
pub fn ground_of(family: &Family) -> u8 {
    let mut count = [0u64; 256];
    for (x, y) in family {
        for &h in x.iter().chain(y).flatten() {
            if let Some(n) = count.get_mut(usize::from(h)) {
                if let Some(more) = n.checked_add(1) {
                    *n = more;
                }
            }
        }
    }
    if count[0] > 0 {
        return 0;
    }
    let mut most: Option<(u64, u8)> = None;
    let mut tied = false;
    for (h, &n) in count.iter().enumerate() {
        let Ok(h) = u8::try_from(h) else { continue };
        match most {
            Some((m, _)) if n < m => {}
            Some((m, _)) if n == m => tied = true,
            _ if n == 0 => {}
            _ => {
                most = Some((n, h));
                tied = false;
            }
        }
    }
    match (most, tied) {
        (Some((_, h)), false) => h,
        _ => 0,
    }
}

thread_local! {
    /// The ground of the family being read on this thread; `None` where no
    /// family's reading is open, and the void is 0.
    static GROUND: std::cell::Cell<Option<u8>> = const { std::cell::Cell::new(None) };
}

/// THE VOID OF THE SCENE BEING READ: the open family's ground, or 0 where
/// none is open, so a reader outside every family reads as it always did.
pub fn ground() -> u8 {
    match GROUND.with(std::cell::Cell::get) {
        Some(g) => g,
        None => 0,
    }
}

/// READ `f` ON THE GROUND OF `family`: the family's ground opened for every
/// perception, speech and replay inside it, UNLESS a family is already open
/// on this thread, whose ground stands (the siblings a perspective derives
/// from, a test spoken through the family's rule, are read on the family's
/// own ground, never on one a subset of the siblings would name). The scope
/// closes as `f` returns.
pub fn on_ground_of<T>(family: &Family, f: impl FnOnce() -> T) -> T {
    match GROUND.with(std::cell::Cell::get) {
        Some(_) => f(),
        None => on_ground(ground_of(family), f),
    }
}

/// READ `f` ON GROUND `g`, whatever was open; the previous scope restored as
/// `f` returns.
pub fn on_ground<T>(g: u8, f: impl FnOnce() -> T) -> T {
    let before = GROUND.with(|c| c.replace(Some(g)));
    let out = f();
    GROUND.with(|c| c.set(before));
    out
}

/// A TASK'S FRAME FIXED ON THE FAMILY'S OWN GROUND: the moments of
/// [`frame_fixed`] weigh the painted sites, those off the ground the train
/// family shows. Every decoder hands its tasks through this, so a task reads
/// one way whatever source it came from.
pub fn frame_fixed_on_ground(task: Task) -> Task {
    let ground = ground_of(&task.train);
    on_ground(ground, || frame_fixed(task))
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
    // THE MOMENTS, per axis and per order 1..=4, over the visible rasters.
    let mut moment = [[0i128; 4]; AXES];
    let visible = task.train.iter().flat_map(|(a, b)| [a, b]).chain(task.test.iter().map(|(a, _)| a));
    for g in visible {
        let e = extents(g);
        for (site, _) in painted_sites(g) {
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
    let reversed: [bool; AXES] = std::array::from_fn(|axis| {
        let [first, _, third, _] = moment[axis];
        match first {
            0 => third < 0,
            f => f < 0,
        }
    });
    let mut order: [usize; AXES] = std::array::from_fn(|axis| axis);
    // A stable ordering: axes whose even moments tie keep the order given.
    #[expect(clippy::disallowed_methods, reason = "rule 4 — a gauge-fixing condition, not a choice among candidates: the axes are ordered by the even moments the scene itself carries (78.54)")]
    order.sort_by_key(|&axis| (moment[axis][1], moment[axis][3]));
    let at = |g: &Vec<Vec<u8>>| reoriented(g, order, reversed);
    Task {
        train: task.train.iter().map(|(a, b)| (at(a), at(b))).collect(),
        test: task.test.iter().map(|(a, b)| (at(a), at(b))).collect(),
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
    let mut total: [Ratio<i128>; AXES] = std::array::from_fn(|_| Ratio::from_integer(0));
    for (a, b) in train {
        let Some((p, q)) = super::family::envelope_dilation(a, b) else { return false };
        // A scene's first moment, per axis, each painted site a unit of mass.
        let moment = |g: &[Vec<u8>]| -> [i128; AXES] {
            let e = extents(g);
            let mut m = [0i128; AXES];
            for (site, _) in painted_sites(g) {
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

/// THE RASTER'S AXES. The carrier is two nested vectors, and every
/// geometric reader here is written ONCE PER AXIS over this count, never an
/// axis by hand. The count is the carrier's, not the data's: a sample
/// carries its own rank (MU-SENSING §2), and this count leaves with the
/// carrier (track 26, S4).
/// FORMAT: the rank of the carrier `Vec<Vec<u8>>`, two levels of nesting.
pub const AXES: usize = 2;

/// A raster's extent on each axis.
pub fn extents(g: &[Vec<u8>]) -> [usize; AXES] {
    [g.len(), raster_width(g)]
}

/// THE SENSOR'S ONE READING OF A RASTER: each painted site (its coordinate
/// on every axis) with its symbol, in canonical address order. The one pass
/// over a scene's sites (law 11's sensor clause); every reader here takes
/// it.
pub fn painted_sites(g: &[Vec<u8>]) -> Vec<([usize; AXES], u8)> {
    let void = ground();
    let mut out = Vec::new();
    for (i, line) in g.iter().enumerate() {
        for (j, &h) in line.iter().enumerate() {
            if h != void {
                out.push(([i, j], h));
            }
        }
    }
    out
}

/// The symbol at a site, the scene's ground (the void) where the site is
/// not in the raster.
pub fn value_at(g: &[Vec<u8>], site: [usize; AXES]) -> u8 {
    match g.get(site[0]).and_then(|line| line.get(site[1])) {
        Some(&h) => h,
        None => ground(),
    }
}

/// A coordinate centred on its extent and doubled, so every centre is an
/// integer: `X = 2x − (n − 1)`.
pub(crate) fn centred(x: usize, n: usize) -> i128 {
    2 * x as i128 - (n as i128 - 1)
}

/// A raster with its axes permuted (`order[j]` is the axis read as the
/// j-th) and reversed where `reversed[axis]`: one pass over its painted
/// sites, the void everywhere else.
fn reoriented(g: &[Vec<u8>], order: [usize; AXES], reversed: [bool; AXES]) -> Vec<Vec<u8>> {
    let e = extents(g);
    let ne: [usize; AXES] = std::array::from_fn(|j| e[order[j]]);
    let mut out: Vec<Vec<u8>> = vec![vec![ground(); ne[1]]; ne[0]];
    for (site, h) in painted_sites(g) {
        let at: [usize; AXES] = std::array::from_fn(|j| {
            let axis = order[j];
            match reversed[axis] {
                true => e[axis] - 1 - site[axis],
                false => site[axis],
            }
        });
        out[at[0]][at[1]] = h;
    }
    out
}

/// A raster → its painted sites, the PERCEPTUAL convention: the scene's
/// ground is omitted (the eye sees only novelty). The walk is the raster's
/// own sites (a scene boundary), not a product of carriers.
pub fn painted_of(g: &[Vec<u8>]) -> Vec<(i64, i64, u8)> {
    let void = ground();
    let mut out = Vec::new();
    for (i, line) in g.iter().enumerate() {
        for (j, &h) in line.iter().enumerate() {
            if h != void {
                out.push((i as i64, j as i64, h));
            }
        }
    }
    out
}

// ── THE SAMPLE BRIDGE (ROADMAP track 26, S4; MU-SENSING §2) ────────────────
//
// A raster reaches the transduction only as a `Sample` read here, and the
// transduction's speech returns to a raster only here. No transduce file
// reads a raster.

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

/// A SPOKEN SAMPLE AS A RASTER: the speech end of the bridge. `None` where
/// the sample is not of rank 2 and width 1, which no raster holds.
pub fn raster_of(s: &Sample) -> Option<Vec<Vec<u8>>> {
    let (AxisReading::Read(lines), AxisReading::Read(width), 2, 1) = (s.extent(0), s.extent(1), s.rank(), s.width()) else {
        return None;
    };
    let bytes: Vec<u8> = s
        .symbols()
        .filter_map(|symbol| match symbol.bytes() {
            &[b] => Some(b),
            _ => None,
        })
        .collect();
    (bytes.len() == lines * width).then(|| bytes.chunks(width).map(<[u8]>::to_vec).collect())
}

/// A PROJECTED RECORD AS THE PLANAR CHAIN `(axis 0, axis 1, before,
/// after)`: the speech end of the bridge for a residue. A change off rank 2
/// or width 1 has no planar link and is left out, which no raster pair
/// produces.
pub fn links_of(changes: &[super::family::transduce::SiteChange]) -> Vec<(i64, i64, u8, u8)> {
    changes
        .iter()
        .filter_map(|c| match (c.site().axes(), c.before().bytes(), c.after().bytes()) {
            (&[i, j], &[before], &[after]) => Some((i, j, before, after)),
            _ => None,
        })
        .collect()
}

/// THE GROUND AS THE SYMBOL A RASTER'S SAMPLE WRITES: one byte, the open
/// family's ground ([`ground`]). The transduction takes the ground as a
/// symbol handed in, never as a code of its own.
pub fn ground_bytes() -> [u8; 1] {
    [ground()]
}

/// [`sample_of`] for a reader that holds no family coordinates for the
/// raster: the sample's place is empty, a place the family has not written,
/// never a coordinate invented for it.
pub fn sample_unplaced(g: &[Vec<u8>]) -> Result<Sample, SampleRefusal> {
    sample_of(g, Place::new(Vec::new()))
}
