// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! Law 11 — THE SENSOR'S GRIDS: a task's grids read as input→output train
//! pairs of A→B demands on painted sites, and the measured frame of a
//! pair. The scene's ground (`ground`: hue 0, or the hue a family is drawn
//! on) is not a painted site; this arm presents the scene and does no
//! cognition.

use serde::Deserialize;

/// A dense ARC grid of hues (0 = background).
pub type Grid = Vec<Vec<u8>>;

/// THE WIDTH OF A GRID. A grid is rectangular — every reader that takes
/// one refuses a ragged one — so row zero is a COORDINATE of the grid and
/// its length is the grid's width, not the length of whichever row arrived
/// first (rule 15). A grid with no row has no sites and measures zero.
///
/// One carrier for it: the same `first().map_or(0, …)` was written out at
/// seven sites, three of them with a different floor.
pub fn grid_width(g: &[Vec<u8>]) -> usize {
    #[expect(
        clippy::disallowed_methods,
        reason = "rule 15 — canonical key order: row zero is the grid's own coordinate, and a rectangular grid's width is its length"
    )]
    g.first().map_or(0, Vec::len)
}

/// THE SCENE'S AMBIENT GROUND Ω (ruled 2026-10-18): the hue
/// the bodies of a family stand on — the void the eye does not see. A
/// family that shows hue 0 on any grid of its demonstration pairs stands on
/// hue 0. A family that never shows it is drawn on a canvas: its ground is
/// the hue of the greatest cell count over those grids, an exact count in
/// ℤ; where two hues tie for it the scene names no ground of its own and
/// the void is hue 0 (`Tie::Hold`, law 10). (The count alone, over every
/// family, read a body larger than its ground — a filled square, a frame
/// grown full — as the ground itself: refined on that reading.) The
/// channels are never relabelled (78.56 II): a hue 7 ground stays hue 7 on
/// every grid, word and wire; Ω only says which hue the eye reads as the
/// void.
pub fn ground_of(family: &[(Grid, Grid)]) -> u8 {
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
    /// family's reading is open, and the void is hue 0.
    static GROUND: std::cell::Cell<Option<u8>> = const { std::cell::Cell::new(None) };
}

/// THE VOID OF THE SCENE BEING READ: the open family's ground, or hue 0
/// where none is open — so a reader outside every family reads as it
/// always did.
pub fn ground() -> u8 {
    match GROUND.with(std::cell::Cell::get) {
        Some(g) => g,
        None => 0,
    }
}

/// READ `f` ON THE GROUND OF `family`: the family's ground opened for every
/// perception, speech and replay inside it — UNLESS a family is already
/// open on this thread, whose ground stands (the siblings a perspective
/// derives from, a test spoken through the family's rule, are read on the
/// family's own ground, never on one a subset of the siblings would name). The scope
/// closes as `f` returns.
pub fn on_ground_of<T>(family: &[(Grid, Grid)], f: impl FnOnce() -> T) -> T {
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

#[derive(Deserialize)]
struct RawPair {
    input: Grid,
    #[serde(default)]
    output: Grid,
}

#[derive(Deserialize)]
struct RawTask {
    #[serde(default)]
    train: Vec<RawPair>,
    #[serde(default)]
    test: Vec<RawPair>,
}

/// THE ARC-AGI-2 CORPUS A READER OPENS (the tests, the algebra's oracle):
/// the `split` directory (`training` or `evaluation`) under
/// `MYCELIUM_CORPUS` where it is set, else under `data/arc-agi-2` at the
/// repository's root. The corpus is not vendored; a reader that finds no
/// corpus says so.
pub fn corpus(split: &str) -> String {
    let root = match std::env::var("MYCELIUM_CORPUS") {
        Ok(r) => r,
        Err(_) => concat!(env!("CARGO_MANIFEST_DIR"), "/data/arc-agi-2").to_string(),
    };
    format!("{root}/{split}")
}

/// A parsed ARC task: the train family (input→output demonstrations) and the test
/// inputs (output may be withheld in evaluation tasks — empty then).
#[derive(Clone)]
pub struct Task {
    pub train: Vec<(Grid, Grid)>,
    pub test: Vec<(Grid, Grid)>,
}

/// Parse a task's JSON (`{"train":[{"input","output"}],"test":[...]}`), its
/// frame fixed where it is a gauge (`frame_fixed`, 78.54, 78.56). Its
/// channels are the detector's own — an absolute boundary (78.56 II).
pub fn parse_task(json: &str) -> Option<Task> {
    let raw: RawTask = serde_json::from_str(json).ok()?;
    let task = Task {
        train: raw.train.into_iter().map(|p| (p.input, p.output)).collect(),
        test: raw.test.into_iter().map(|p| (p.input, p.output)).collect(),
    };
    // The frame's moments weigh the painted sites: those off the family's
    // own ground.
    let ground = ground_of(&task.train);
    Some(on_ground(ground, || frame_fixed(task)))
}

/// 78.54, 78.56 — THE LATTICE'S FRAME IS A GAUGE WHERE THE TASK CARRIES NO
/// POLAR VECTOR. A task turned by 90°, 180° or 270°, or mirrored, as a
/// whole — every grid of it — is the same task when its centre of mass does
/// not move from input to output (Δr_CM = 0, the O(2) singlet): the frame
/// the sensor happened to read it in is a coordinate choice, and a word that
/// carried it would make a mind that learned a motion in one frame blind to
/// it in another. Where Δr_CM ≠ 0 the orientation is physical and kept.
///
/// The frame is fixed at the sensor and nowhere after —
/// READ, AXIS BY AXIS, OFF THE SCENE'S OWN MOMENTS, never by enumerating the
/// lattice's symmetry group (2ⁿ·n! poses in n axes; rules 3 and 4). Over the
/// VISIBLE grids (train pairs and test inputs; a withheld output never
/// moves the frame), each painted site a unit of mass, in centred
/// coordinates:
/// - an axis is REVERSED where its first non-zero odd moment (orders 1, 3)
///   is negative — which is what a reflection along it changes;
/// - the axes are ORDERED by the axes' even moments (orders 2, 4) — which is
///   what a permutation of the axes changes.
///
/// These are a gauge-fixing condition — the representative the geometry
/// itself names — never a candidate chosen by a scalar. They are hue-blind.
/// Where an axis's odd moments all vanish, or two axes' even moments tie,
/// the scene is symmetric under that move as far as these orders read, and
/// the frame given is kept (law 10: held, never broken by an arbitrary
/// choice). The channels are never relabelled: the detector is a Dirichlet
/// boundary, the laboratory's anchor for every mind (78.56 II).
pub fn frame_fixed(task: Task) -> Task {
    // 78.56 — A TASK WITH A POLAR VECTOR HAS A PHYSICAL ORIENTATION: its
    // momentum breaks the lattice's symmetry, and its frame is kept.
    if !momentum_vanishes(&task.train) {
        return task;
    }
    // THE MOMENTS, per axis and per order 1..=4, over the visible grids.
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
    let at = |g: &Grid| reoriented(g, order, reversed);
    Task {
        train: task.train.iter().map(|(a, b)| (at(a), at(b))).collect(),
        test: task.test.iter().map(|(a, b)| (at(a), at(b))).collect(),
    }
}

/// THE SENSOR LATTICE'S AXES. A `Grid` is read on two — rows, columns — and
/// every geometric reader here is written ONCE PER AXIS over this count,
/// never an axis by hand: a lattice of more axes changes this constant and
/// the grid's own storage, and nothing else.
pub const AXES: usize = 2;

/// A grid's extent on each lattice axis.
pub fn extents(g: &Grid) -> [usize; AXES] {
    [g.len(), grid_width(g)]
}

/// THE SENSOR'S ONE READING OF A GRID: each painted site — its coordinate on
/// every axis — with its hue, in the lattice's own order. The one pass over
/// a scene's sites (law 11's sensor clause); every reader here takes it.
pub fn painted_sites(g: &Grid) -> Vec<([usize; AXES], u8)> {
    let void = ground();
    let mut out = Vec::new();
    for (r, row) in g.iter().enumerate() {
        for (c, &h) in row.iter().enumerate() {
            if h != void {
                out.push(([r, c], h));
            }
        }
    }
    out
}

/// The hue at a site, the scene's ground (the void) where the site is not
/// in the grid.
pub fn hue_at(g: &Grid, site: [usize; AXES]) -> u8 {
    match g.get(site[0]).and_then(|row| row.get(site[1])) {
        Some(&h) => h,
        None => ground(),
    }
}

/// A coordinate centred on its extent and doubled, so every centre is an
/// integer: `X = 2x − (n − 1)`.
fn centred(x: usize, n: usize) -> i128 {
    2 * x as i128 - (n as i128 - 1)
}

/// A grid with its axes permuted (`order[j]` is the axis read as the j-th)
/// and reversed where `reversed[axis]`: one pass over its painted sites,
/// the void everywhere else.
fn reoriented(g: &Grid, order: [usize; AXES], reversed: [bool; AXES]) -> Grid {
    let e = extents(g);
    let ne: [usize; AXES] = std::array::from_fn(|j| e[order[j]]);
    let mut out: Grid = vec![vec![ground(); ne[1]]; ne[0]];
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

/// 78.56 — THE DILATION BETWEEN TWO ENVELOPES. A grid is enveloped by its
/// circumscribed sphere — centre the extent's midpoint, r² = Σ eᵢ²/4 over
/// its axes — and a change of extent is the dilation about the centres
/// carrying one envelope onto the other, by s = r_to / r_from. It is exact
/// only where r²_to / r²_from is a rational square: answered as `s = p/q`
/// in lowest terms, `None` where s is irrational (78.45: no approximation)
/// or an extent is empty. Two grids of one extent: 1/1.
pub fn envelope_dilation(from: &Grid, to: &Grid) -> Option<(i64, i64)> {
    let r2 = |g: &Grid| -> i64 { extents(g).iter().map(|&e| (e as i64) * (e as i64)).sum() };
    let (a, b) = (r2(to), r2(from));
    if a == 0 || b == 0 {
        return None;
    }
    let g = gcd_i64(a, b);
    let (a, b) = (a / g, b / g);
    let (p, q) = (exact_root(a)?, exact_root(b)?);
    Some((p, q))
}

/// Where a site of `from` stands in `to` under the envelopes' dilation
/// (`envelope_dilation`, `s = p/q`), read in centred coordinates doubled
/// so every centre is an integer: `X = 2r − (h − 1)`. `None` where the
/// carried point is not a site of `to` — between sites, or outside it.
pub fn carried_site(from: &Grid, to: &Grid, (p, q): (i64, i64), site: [usize; AXES]) -> Option<[usize; AXES]> {
    let (e_from, e_to) = (extents(from), extents(to));
    let (p, q) = (i128::from(p), i128::from(q));
    let mut out = [0usize; AXES];
    for axis in 0..AXES {
        let scaled = centred(site[axis], e_from[axis]).checked_mul(p)?;
        if scaled % q != 0 {
            return None;
        }
        let doubled = scaled / q + (e_to[axis] as i128 - 1);
        if doubled % 2 != 0 || doubled < 0 {
            return None;
        }
        let at = usize::try_from(doubled / 2).ok()?;
        if at >= e_to[axis] {
            return None;
        }
        out[axis] = at;
    }
    Some(out)
}

/// 78.56 — WHETHER THE TASK CARRIES NO POLAR VECTOR: its momentum — the
/// mass-weighted displacement from input to output, `Σ_out X − s·Σ_in X`
/// in centred coordinates across each pair's envelope dilation, summed
/// over the train pairs, every painted site a unit of mass — is exactly
/// zero. `false` where it is not, and where any pair's dilation is
/// irrational (its momentum unread: the orientation is held physical, so
/// the frame is never un-fixed by a guess).
pub fn momentum_vanishes(train: &[(Grid, Grid)]) -> bool {
    use num_rational::Ratio;
    let mut total: [Ratio<i128>; AXES] = std::array::from_fn(|_| Ratio::from_integer(0));
    for (a, b) in train {
        let Some((p, q)) = envelope_dilation(a, b) else { return false };
        // A scene's first moment, per axis, each painted site a unit of mass.
        let moment = |g: &Grid| -> [i128; AXES] {
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

/// The greatest common divisor of two positive integers.
fn gcd_i64(a: i64, b: i64) -> i64 {
    let (mut a, mut b) = (a.abs(), b.abs());
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

/// The exact square root of a non-negative integer, or `None` where it is
/// not a perfect square: Newton's iteration on the integers, which strictly
/// decreases from `n` to ⌊√n⌋ — O(log n) steps, bounded by its own descent
/// (rule 10), never a walk up from zero.
fn exact_root(n: i64) -> Option<i64> {
    if n < 0 {
        return None;
    }
    if n < 2 {
        return Some(n);
    }
    let mut x = n;
    let mut next = (x + 1) / 2;
    while next < x {
        x = next;
        next = (x + n / x) / 2;
    }
    (x * x == n).then_some(x)
}

/// A grid → its painted sites, the PERCEPTUAL convention: the scene's
/// ground is omitted (the eye sees only novelty). The walk is the grid's own
/// sites (a scene boundary), not a product of carriers.
pub fn painted_of_grid(g: &Grid) -> Vec<(i64, i64, u8)> {
    let void = ground();
    let mut out = Vec::new();
    for (r, row) in g.iter().enumerate() {
        for (c, &h) in row.iter().enumerate() {
            if h != void {
                out.push((r as i64, c as i64, h));
            }
        }
    }
    out
}

/// The μ⃗ of one train pair: the coboundary's lane measure. T is not on the chain.
/// None when the coboundary is unmeasured.
pub fn pair_magnitude(a: &Grid, b: &Grid) -> Option<[u64; 4]> {
    let ca = painted_of_grid(a);
    let cb = painted_of_grid(b);
    crate::transducer::germinate::germinate(&ca, &cb)
        .map(|g| crate::transducer::element::measure_lanes(&g))
}

/// Lane-wise accumulation of one pair's μ⃗ into a bundle's. The pairs of a
/// bundle are DISJOINT SCENES, so a bundle's measure on a lane is the sum of
/// its pairs' — chapter 12.6: "the lane-wise sum of the per-pair pressures,
/// each pair measured by `measure_lanes` on its own residual, the pairs
/// being disjoint scenes; never a maximum over pairs". A maximum reports a
/// bundle as no larger than its largest pair, which is how a genesis bond
/// measured from a corpus came out too small to cover a single pair of it.
/// Lanes never meet: each accumulates alone. `None` on overflow — a lane
/// that cannot hold its own sum is unmeasured, never capped (rule 11).
pub fn cover_join(cover: &mut [u64; 4], mag: &[u64; 4]) -> Option<()> {
    for l in 0..4 {
        cover[l] = cover[l].checked_add(mag[l])?;
    }
    Some(())
}

/// Genesis bond of one family: the lane-wise measure of its train pairs,
/// which is the work the family presents. The train is the family's
/// boundary. `None` when no pair is measured or a lane overflows.
pub fn family_cover(train: &[(Grid, Grid)]) -> Option<[u64; 4]> {
    let mut cover = [0u64; 4];
    let mut any = false;
    for (a, b) in train {
        let Some(mag) = pair_magnitude(a, b) else {
            continue;
        };
        if mag.iter().all(|&x| x == 0) {
            continue;
        }
        any = true;
        cover_join(&mut cover, &mag)?;
        // H carries the pair's own coboundary extent, accumulated with the
        // rest: the same disjoint-scene rule as every other lane.
        let chain = crate::transducer::germinate::germinate(&painted_of_grid(a), &painted_of_grid(b))?;
        cover[3] = cover[3].checked_add(chain.len() as u64)?;
    }
    any.then_some(cover)
}

// ── #97: THE CODOMAIN APERTURE — cross-complex chains ────────────────────────
//
// A pair of complexes K_in (h_in × w_in) and K_out (h_out × w_out) is related
// by the exact frame map ρ = (h_out/h_in, w_out/w_in) ∈ ℚ². The cross-frame
// coboundary is supported on the codomain complex: every site of K_out reads
// the input through ρ⁻¹ (the input site containing the output site's min
// corner — the substrate's framing convention, the same min-corner quotient
// every chain is centered by) and differs, or not, from the output. No common refinement
// is materialized: the support is bounded by |K_out|, and ρ rides as the
// word's frame fiber. Uniform for every ratio including (1,1), which is the
// planar delta bit-for-bit. Banned surface: no branch anywhere on ratio
// VALUES — the map is total.

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 { a } else { gcd(b, a % b) }
}

/// THE MEASURED FRAME of a pair: four exact counts, never a ratio and never
/// a guess. The ratio is derived from them by `frame_ratio`, and a word at a
/// frame that is not (1,1) is a DIFFERENT word from its planar shadow.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Frame {
    pub h_in: usize,
    pub w_in: usize,
    pub h_out: usize,
    pub w_out: usize,
}

impl Frame {
    /// Identity: no scale. Unframed speech uses this so its key matches a
    /// planar intake.
    pub const IDENTITY: Frame = Frame { h_in: 1, w_in: 1, h_out: 1, w_out: 1 };

    /// The frame of a pair whose extents are known.
    pub const fn new(h_in: usize, w_in: usize, h_out: usize, w_out: usize) -> Frame {
        Frame { h_in, w_in, h_out, w_out }
    }
}

pub fn frame_ratio(f: &Frame) -> ((usize, usize), (usize, usize)) {
    let Frame { h_in: hi, w_in: wi, h_out: ho, w_out: wo } = *f;
    let (gh, gw) = (gcd(ho, hi), gcd(wo, wi));
    let hr = match (ho.checked_div(gh), hi.checked_div(gh)) {
        (Some(o), Some(i)) => (o, i),
        _ => (0, 0),
    };
    let wr = match (wo.checked_div(gw), wi.checked_div(gw)) {
        (Some(o), Some(i)) => (o, i),
        _ => (0, 0),
    };
    (hr, wr)
}


/// WHAT REFINING A PAIR YIELDS.
pub struct Refined {
    /// The coboundary: the sites that changed, each with the hue it came
    /// from and the hue it went to.
    pub delta: Vec<(i64, i64, u8, u8)>,
    /// The input pulled back onto the output complex — the scene π is
    /// evaluated over, not the input as given.
    pub pulled_back: Grid,
    /// The frame it was read in.
    pub frame: Frame,
}

/// THE CROSS-COMPLEX COBOUNDARY on the codomain: the input pulled back along
/// ρ⁻¹ onto K_out, then the ordinary per-site difference. Returns (chain on
/// K_out, the pulled-back input — the scene π evaluates over, the measured
/// frame). Total for all dims ≥ 1; equal frames yield exactly the planar
/// delta. Support ≤ |K_out|.
pub fn refine_delta(a: &Grid, b: &Grid) -> Option<Refined> {
    let (hi, wi) = (a.len(), grid_width(a));
    let (ho, wo) = (b.len(), grid_width(b));
    if hi == 0 || wi == 0 || ho == 0 || wo == 0 {
        return None;
    }
    if a.iter().any(|r| r.len() != wi) || b.iter().any(|r| r.len() != wo) {
        return None; // ragged grids carry no complex
    }
    let refn = crate::transducer::family::refine::CommonRefinement::of(hi, wi, ho, wo)?;
    let (pulled_back, _) = refn.refined(a, b)?;
    let chain = refn.bipartite_cochain(a, b)?;
    Some(Refined { delta: chain, pulled_back, frame: Frame::new(hi, wi, ho, wo) })
}




#[cfg(test)]
mod tests {
    use super::*;

    /// 78.56 — THE ENVELOPES' DILATION: two grids of one extent correspond
    /// site for site; 3×3 onto 9×9 is s = 3, carrying each cell's centre to
    /// the centre of its 3×3 block, never to a corner; an extent whose
    /// envelope ratio is no rational square corresponds nowhere (78.45); and
    /// a point that lands between sites carries nothing.
    #[test]
    fn envelope_dilation_carries_sites_between_extents() {
        let grid = |h: usize, w: usize| -> Grid { vec![vec![1; w]; h] };
        assert_eq!(envelope_dilation(&grid(3, 4), &grid(3, 4)), Some((1, 1)));
        assert_eq!(envelope_dilation(&grid(3, 3), &grid(9, 9)), Some((3, 1)));
        assert_eq!(carried_site(&grid(3, 3), &grid(9, 9), (3, 1), [1, 1]), Some([4, 4]), "centre to centre");
        assert_eq!(carried_site(&grid(3, 3), &grid(9, 9), (3, 1), [0, 0]), Some([1, 1]), "a cell to its block's centre");
        assert_eq!(envelope_dilation(&grid(1, 1), &grid(1, 2)), None, "√(5/2) is no rational");
        assert_eq!(carried_site(&grid(2, 2), &grid(4, 4), (2, 1), [0, 0]), None, "between sites");
        // The momentum: a recolour in place carries none; a drift one way does.
        let still = vec![(vec![vec![1, 1, 0]], vec![vec![2, 2, 0]])];
        let drift = vec![(vec![vec![1, 1, 0]], vec![vec![0, 1, 1]])];
        assert!(momentum_vanishes(&still));
        assert!(!momentum_vanishes(&drift));
    }


    /// A task's JSON parses into its train pairs and test inputs, and a
    /// withheld test output is empty.
    #[test]
    fn parse_task_reads_train_and_test_pairs() {
        let json = r#"{"train":[{"input":[[0,1],[2,0]],"output":[[1,0],[0,2]]}],"test":[{"input":[[3]]}]}"#;
        let t = parse_task(json).expect("parses");
        assert_eq!(t.train.len(), 1);
        assert_eq!(t.test.len(), 1);
        assert_eq!(t.train[0].0, vec![vec![0, 1], vec![2, 0]]);
        assert!(t.test[0].1.is_empty(), "a withheld test output is empty");
    }

    /// A grid's painted sites omit the background, each kept as row,
    /// column and hue.
    #[test]
    fn painted_of_grid_omits_background() {
        // hue-0 sites are dropped; painted sites keep (row, col, hue).
        let painted = painted_of_grid(&vec![vec![0, 1], vec![2, 0]]);
        assert_eq!(painted, vec![(0, 1, 1), (1, 0, 2)], "only novelty (non-background)");
        assert!(painted_of_grid(&vec![vec![0, 0], vec![0, 0]]).is_empty(), "all-background → no painted site");
    }

    /// 12.6: a bundle's pressure is the lane-wise SUM over its pairs, which
    /// are disjoint scenes — never a maximum. A maximum would report this
    /// two-pair family as no larger than its wider pair, so a bond measured
    /// from it could not cover the family it was measured from.
    #[test]
    fn family_cover_sums_pairs_lane_wise() {
        let small = (vec![vec![1, 1]], vec![vec![2, 2]]);
        let wide = (vec![vec![1, 1, 1]], vec![vec![2, 2, 2]]);
        let cover = family_cover(&[small.clone(), wide.clone()]).expect("measured");
        let a = pair_magnitude(&small.0, &small.1).expect("small");
        let b = pair_magnitude(&wide.0, &wide.1).expect("wide");
        for l in 0..3 {
            assert_eq!(cover[l], a[l] + b[l], "lane {l} sums the disjoint scenes");
            assert!(cover[l] >= a[l].max(b[l]), "and therefore covers each pair");
        }
        let h = |p: &(Grid, Grid)| {
            crate::transducer::germinate::germinate(&painted_of_grid(&p.0), &painted_of_grid(&p.1))
                .map(|c| c.len() as u64)
                .expect("measured")
        };
        assert_eq!(
            cover[3],
            a[3] + b[3] + h(&small) + h(&wide),
            "H accumulates the coboundary extent with the rest"
        );
        assert!(family_cover(&[]).is_none(), "an empty family is unmeasured, not a zero bond");
    }

    /// The Euclidean gcd is exact and the frame ratio is total, an empty
    /// frame reading `((0, 0), (0, 0))`.
    #[test]
    fn frame_ratio_total_over_exact_gcd() {
        assert_eq!(gcd(0, 0), 0);
        assert_eq!(gcd(12, 8), 4);
        assert_eq!(gcd(7, 0), 7);
        assert_eq!(gcd(0, 5), 5);
        assert_eq!(frame_ratio(&Frame::new(0, 0, 0, 0)), ((0, 0), (0, 0)));
        assert_eq!(frame_ratio(&Frame::new(4, 6, 2, 3)), ((1, 2), (1, 2)));
    }
}

