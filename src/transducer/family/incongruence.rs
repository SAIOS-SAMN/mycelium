// 78.79 — THE INTERPRETER FOR INCONGRUENCE (an instrument, deciding nothing):
// two bodies no motion makes the same are related by what they are made of —
// the constructors they share, those accreted, those extinguished — and a
// subject is defined by holding its readings together and contrasting them,
// never by dissolving them into one.
//
//! 1. THE CONSTRUCTOR DIFFERENCE (one change). An output body standing on
//!    exactly one input body (78.78's single carriage, 62% / 91% of what is
//!    left unsaid) overlaps it, so its core motion is the identity in place.
//!    Its constructors are its compartments' rounds, matched exactly by their
//!    coordinates (keyed, never searched): SHARED (the core), ACCRETED (the
//!    white hole's channel — growth, bridge), EXTINGUISHED (the black hole's —
//!    carving, hollowing), and the shared ones RECOLOURED (the hue fibre). The
//!    accretion is read as light (on a straight run out of the input body,
//!    78.77) and the extinction as carving (interior to the input body).
//! 2. THE CONTRAST PAIR. Two readings of one subject held together, each
//!    whole: what they share is DRAG (coherence), what differs LIFT (the
//!    contrast); neither absorbs the other.
//! 3. THE COHERENCE MANIFOLD. Per subject — the input bodies of one chart
//!    across a family's pairs — every reading held, written relative to its
//!    input body's exact barycentre. Its CORE is what every reading shares;
//!    its EDGES what some readings hold and others do not. Three readings of
//!    its sharpness, kept apart:
//!    - RESOLUTION: the quantum of its light — the greatest common length of
//!      the accretion's runs, refined as readings arrive (F4's law);
//!    - PHASE: luminal (open — its runs leave by the frame's edge) or
//!      holonomic (closed — they end at a body), per reading;
//!    - EDGES: the constructors that contrast.
//!
//!    Each new reading is counted by what it moved: refined, closed,
//!    sharpened an edge, or recognized.
//! 4. JUDGED BY THE PLANE: the core difference replayed on every reading's
//!    input body — reproduces its output, or does not.
//! 5. 78.80 — A BODY BY ITS MOMENTS, NOT ITS BOUNDARY. The confluence sphere
//!    C = ΣPₛ of a body's compartments carries its MASS M = −C·e∞ (volume:
//!    the compartment count), its centre (the barycentre) and its SPREAD
//!    −C² = Σ_{s<t} d²ₛₜ (the second moment), all exact integers; its
//!    DENSITY ρ = M² / spread is one rational, and per hue the mass is a
//!    vector. Two bodies no motion makes congruent may agree on every
//!    moment. The second moment by axes is the INERTIA FORM, and its ratio
//!    the body's DIRECTION. A recolour is then asked which reading DECIDES
//!    it: a key — volume, density, hue-mass, shape, direction, or RELATIONAL,
//!    the body's rank by mass or density among the bodies of its pair —
//!    decides a reading when the other readings of its family under the same
//!    key agree on one hue map, and the prediction is judged by replay on the
//!    input body.
//! 6. The single changes of a family's remainder, with these keys, are one
//!    reading ([`single_changes`], [`decide`]) shared by the instrument and
//!    the voice's interpreter (`interpret`).

use super::*;
use num_integer::Integer;
use num_rational::BigRational;
use num_traits::Zero;

type Q = BigRational;

/// One compartment: its site and hue.
pub type Cell = (i64, i64, u8);

/// ONE CHANGE, BY ITS CONSTRUCTORS.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Difference {
    pub shared: u64,
    pub accreted: Vec<Cell>,
    pub extinguished: Vec<Cell>,
    /// Shared sites whose hue changed: (site, from, to).
    pub recoloured: Vec<(i64, i64, u8, u8)>,
    /// Accreted sites on a straight run out of the input body, and the runs'
    /// lengths with how each ended: at the frame's edge (open) or at a body.
    pub on_light: u64,
    pub runs: Vec<(u64, bool)>,
    /// Extinguished sites interior to the input body.
    pub carved: u64,
}

impl Difference {
    /// No constructor accreted or extinguished: the change is kinematic (here,
    /// in place: a recolour or nothing).
    pub fn kinematic(&self) -> bool {
        self.accreted.is_empty() && self.extinguished.is_empty()
    }
}

/// THE CONSTRUCTOR DIFFERENCE between an input body and the output body that
/// stands on it, in place (the core motion the identity).
pub fn difference(input: &[Cell], output: &[Cell], frame: &Grid) -> Difference {
    let at = |cells: &[Cell], r: i64, c: i64| cells.iter().find(|x| x.0 == r && x.1 == c).map(|x| x.2);
    let mut d = Difference { shared: 0, accreted: Vec::new(), extinguished: Vec::new(), recoloured: Vec::new(), on_light: 0, runs: Vec::new(), carved: 0 };
    for &(r, c, h) in output {
        match at(input, r, c) {
            Some(g) => {
                d.shared += 1;
                if g != h {
                    d.recoloured.push((r, c, g, h));
                }
            }
            None => d.accreted.push((r, c, h)),
        }
    }
    for &(r, c, h) in input {
        if at(output, r, c).is_none() {
            d.extinguished.push((r, c, h));
            let interior = [(0, 1), (0, -1), (1, 0), (-1, 0)].iter().all(|(dr, dc)| at(input, r + dr, c + dc).is_some());
            d.carved += u64::from(interior);
        }
    }
    // The accretion as light: from each input cell along each of the eight
    // null directions, the run of accreted cells starting next to it.
    let mut lit: Vec<(i64, i64)> = Vec::new();
    for &(r, c, _) in input {
        for (dr, dc) in super::luminal::DIRECTIONS {
            let mut t = 1i64;
            while d.accreted.iter().any(|x| x.0 == r + t * dr && x.1 == c + t * dc) {
                t += 1;
            }
            if t > 1 {
                for k in 1..t {
                    let s = (r + k * dr, c + k * dc);
                    if !lit.contains(&s) {
                        lit.push(s);
                    }
                }
                let (er, ec) = (r + t * dr, c + t * dc);
                let open = usize::try_from(er).ok().zip(usize::try_from(ec).ok()).and_then(|(ur, uc)| frame.get(ur)?.get(uc)).is_none();
                d.runs.push(((t - 1) as u64, open));
            }
        }
    }
    d.on_light = lit.len() as u64;
    d
}

fn barycentre(cells: &[Cell]) -> Option<(Q, Q)> {
    let n = Q::from_integer(i64::try_from(cells.len()).ok()?.into());
    if n.is_zero() {
        return None;
    }
    let (r, c) = cells.iter().fold((Q::zero(), Q::zero()), |(a, b), x| (a + Q::from_integer(x.0.into()), b + Q::from_integer(x.1.into())));
    Some((r / &n, c / n))
}

/// A difference written relative to its input body's barycentre: each
/// accreted and extinguished constructor by its exact offset and hue, the
/// recolours by their hue pair — the reading a subject's readings compare by.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Relative {
    pub accreted: Vec<(Q, Q, u8)>,
    pub extinguished: Vec<(Q, Q, u8)>,
    pub recolours: Vec<(u8, u8)>,
}

pub fn relative(input: &[Cell], d: &Difference) -> Option<Relative> {
    let (br, bc) = barycentre(input)?;
    let off = |x: &Cell| (Q::from_integer(x.0.into()) - &br, Q::from_integer(x.1.into()) - &bc, x.2);
    let mut accreted: Vec<(Q, Q, u8)> = d.accreted.iter().map(off).collect();
    let mut extinguished: Vec<(Q, Q, u8)> = d.extinguished.iter().map(off).collect();
    let mut recolours: Vec<(u8, u8)> = d.recoloured.iter().map(|x| (x.2, x.3)).collect();
    accreted.sort();
    extinguished.sort();
    recolours.sort_unstable();
    recolours.dedup();
    Some(Relative { accreted, extinguished, recolours })
}

/// 2 — TWO READINGS HELD TOGETHER: what they share (drag, coherence) and what
/// each holds that the other does not (lift, contrast) — both kept whole.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Contrast {
    pub shared: u64,
    pub only_first: u64,
    pub only_second: u64,
}

pub fn contrast(a: &Relative, b: &Relative) -> Contrast {
    let all_a: Vec<&(Q, Q, u8)> = a.accreted.iter().chain(&a.extinguished).collect();
    let all_b: Vec<&(Q, Q, u8)> = b.accreted.iter().chain(&b.extinguished).collect();
    let shared = all_a.iter().filter(|x| all_b.contains(x)).count() as u64;
    Contrast { shared, only_first: all_a.len() as u64 - shared, only_second: all_b.len() as u64 - shared }
}

/// What one reading moved when it joined its subject's manifold.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Moved {
    pub refined: bool,
    pub closed: bool,
    pub sharpened: bool,
    pub recognized: bool,
}

/// 3 — ONE SUBJECT'S COHERENCE MANIFOLD: its readings held, its core and its
/// edges, its resolution and phase, and what each reading moved.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Manifold {
    pub readings: Vec<Relative>,
    /// The constructors every reading shares (accreted and extinguished), and
    /// the recolours every reading makes.
    pub core: Vec<(Q, Q, u8)>,
    pub core_recolours: Vec<(u8, u8)>,
    /// Constructors some readings hold and others do not.
    pub edges: u64,
    /// The quantum of its light: the gcd of every accretion run's length.
    pub quantum: Option<u64>,
    /// Readings whose runs all end at a body (holonomic), and those with a run
    /// leaving by the edge (luminal).
    pub holonomic: u64,
    pub luminal: u64,
    pub moved: Vec<Moved>,
}

/// BUILD A SUBJECT'S MANIFOLD from its readings in the order they came, each
/// with its runs.
pub fn manifold(readings: &[(Relative, Vec<(u64, bool)>)]) -> Manifold {
    let mut m = Manifold::default();
    for (rel, runs) in readings {
        let mut moved = Moved::default();
        let all = |r: &Relative| -> Vec<(Q, Q, u8)> { r.accreted.iter().chain(&r.extinguished).cloned().collect() };
        let mine = all(rel);
        moved.recognized = m.readings.iter().any(|h| h == rel);
        // The core narrows to what this reading shares; anything it drops or
        // brings beyond the core is an edge sharpened.
        let core_before = m.core.len();
        match m.readings.is_empty() {
            true => {
                m.core = mine.clone();
                m.core_recolours = rel.recolours.clone();
            }
            false => {
                m.core.retain(|x| mine.contains(x));
                m.core_recolours.retain(|x| rel.recolours.contains(x));
            }
        }
        moved.sharpened = !m.readings.is_empty() && (m.core.len() < core_before || mine.len() > m.core.len());
        // Resolution: the quantum refines where this reading's runs bring a
        // length the held quantum does not divide.
        let before = m.quantum;
        for (len, _) in runs {
            m.quantum = Some(match m.quantum {
                None => *len,
                Some(q) => q.gcd(len),
            });
        }
        moved.refined = matches!((before, m.quantum), (Some(b), Some(a)) if a < b);
        // Phase: every run of this reading ends at a body — the light held —
        // or one leaves by the edge.
        match (runs.is_empty(), runs.iter().all(|(_, open)| !open)) {
            (true, _) => {}
            (false, true) => {
                m.holonomic += 1;
                moved.closed = m.holonomic == 1;
            }
            (false, false) => m.luminal += 1,
        }
        m.readings.push(rel.clone());
        m.moved.push(moved);
    }
    let union: Vec<(Q, Q, u8)> = m.readings.iter().flat_map(|r| r.accreted.iter().chain(&r.extinguished).cloned()).collect();
    let mut distinct = union.clone();
    distinct.sort();
    distinct.dedup();
    m.edges = distinct.iter().filter(|x| !m.core.contains(x)).count() as u64;
    m
}

/// 4 — JUDGED BY THE PLANE: the manifold's core replayed on one reading's
/// input body — its core accretion laid down, its core extinction removed,
/// its core recolours applied — against the output body. Exact.
pub fn reproduces(m: &Manifold, input: &[Cell], output: &[Cell]) -> Option<bool> {
    let (br, bc) = barycentre(input)?;
    let mut spoken: Vec<Cell> = input.to_vec();
    for (dr, dc, h) in &m.core {
        let (r, c) = (&br + dr, &bc + dc);
        if !r.is_integer() || !c.is_integer() {
            return Some(false);
        }
        let (r, c) = (r.to_integer().try_into().ok()?, c.to_integer().try_into().ok()?);
        match spoken.iter().position(|x| x.0 == r && x.1 == c) {
            // An extinguished constructor is on the input: it leaves.
            Some(i) => {
                spoken.remove(i);
            }
            None => spoken.push((r, c, *h)),
        }
    }
    for x in spoken.iter_mut() {
        if let Some((_, to)) = m.core_recolours.iter().find(|(from, _)| *from == x.2) {
            x.2 = *to;
        }
    }
    let mut a = spoken;
    let mut b = output.to_vec();
    a.sort_unstable();
    b.sort_unstable();
    Some(a == b)
}

/// 5 — A BODY'S MOMENTS: its mass (volume), its spread (−C², the sum of
/// squared separations over its pairs of compartments), its second moment by
/// axes (the inertia form, whose ratio is the body's direction) and its mass
/// per hue, sorted by hue. No boundary is read.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Moments {
    pub mass: u64,
    pub spread: i128,
    /// M²·the second central moment: rows, columns, and their product — all
    /// integers; `spread` is the first two summed.
    pub inertia: [i128; 3],
    pub hues: Vec<(u8, u64)>,
}

impl Moments {
    /// DENSITY, ρ = M² / spread: exact, dimensionless. A single compartment
    /// has no spread and its density is refused, never made infinite.
    pub fn density(&self) -> Option<Q> {
        (self.spread != 0).then(|| Q::new((i128::from(self.mass) * i128::from(self.mass)).into(), self.spread.into()))
    }

    /// DIRECTION: the inertia form up to scale — its three integers divided
    /// by their greatest common divisor. Two strokes of one direction and
    /// different lengths read alike; a diagonal and its mirror differ in the
    /// product's sign. A point's form is zero, and reads as itself.
    pub fn direction(&self) -> [i128; 3] {
        let g = self.inertia.iter().fold(0i128, |g, x| g.gcd(x));
        match g {
            0 => self.inertia,
            g => self.inertia.map(|x| x / g),
        }
    }
}

/// THE MOMENTS OF A BODY, from the sums its confluence sphere carries:
/// spread = M·Σ|x|² − |Σx|², which is Σ_{s<t} d²ₛₜ exactly; the inertia
/// form likewise by axes.
pub fn moments(cells: &[Cell]) -> Moments {
    let mass = cells.len() as u64;
    let (sr, sc, srr, scc, src) = cells.iter().fold((0i128, 0i128, 0i128, 0i128, 0i128), |(a, b, p, q, x), cell| {
        let (r, c) = (i128::from(cell.0), i128::from(cell.1));
        (a + r, b + c, p + r * r, q + c * c, x + r * c)
    });
    let m = i128::from(mass);
    let inertia = [m * srr - sr * sr, m * scc - sc * sc, m * src - sr * sc];
    let spread = inertia[0] + inertia[1];
    let mut hues: Vec<(u8, u64)> = Vec::new();
    for x in cells {
        match hues.iter_mut().find(|(h, _)| *h == x.2) {
            Some((_, n)) => *n += 1,
            None => hues.push((x.2, 1)),
        }
    }
    hues.sort_unstable();
    Moments { mass, spread, inertia, hues }
}

/// A hue map: the recolours of one reading, each source hue once.
pub type HueMap = Vec<(u8, u8)>;

/// WHAT A KEY DECIDES, LEAVE ONE OUT: for each reading, the other readings
/// under its key, their hue maps joined into one map. The join is refused
/// where two of them send one hue to two (the key does not decide) and
/// absent where no other reading shares the key (nothing to decide by).
pub fn decided<K: PartialEq>(readings: &[(K, HueMap)]) -> Vec<Option<Option<HueMap>>> {
    readings
        .iter()
        .enumerate()
        .map(|(i, (key, _))| {
            let others: Vec<&HueMap> = readings.iter().enumerate().filter(|(j, (k, _))| *j != i && k == key).map(|(_, (_, m))| m).collect();
            if others.is_empty() {
                return None;
            }
            let mut joined: HueMap = Vec::new();
            for &(from, to) in others.into_iter().flatten() {
                match joined.iter().find(|(f, _)| *f == from) {
                    Some((_, t)) if *t != to => return Some(None),
                    Some(_) => {}
                    None => joined.push((from, to)),
                }
            }
            joined.sort_unstable();
            Some(Some(joined))
        })
        .collect()
}

/// A HUE MAP REPLAYED on an input body against the output body: exact.
pub fn recolours_to(map: &[(u8, u8)], input: &[Cell], output: &[Cell]) -> bool {
    let mut a: Vec<Cell> = input.iter().map(|&(r, c, h)| (r, c, map.iter().find(|(f, _)| *f == h).map_or(h, |(_, t)| *t))).collect();
    let mut b = output.to_vec();
    a.sort_unstable();
    b.sort_unstable();
    a == b
}

/// 6 — ONE SINGLE CHANGE OF A FAMILY'S REMAINDER: an unsaid output body
/// standing on exactly one input body, read by its constructors and by the
/// input body's moments — its own, and RELATIONAL: where it stands among the
/// bodies of its pair by mass and by density.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Change {
    pub pair: usize,
    pub input: Vec<Cell>,
    pub output: Vec<Cell>,
    pub difference: Difference,
    pub moments: Moments,
    pub output_moments: Moments,
    /// The input body's chart key (its shape).
    pub shape: Vec<u8>,
    /// Bodies of its pair's input heavier and lighter than it.
    pub mass_rank: (u64, u64),
    /// Bodies denser and sparser; refused where its own density is.
    pub density_rank: Option<(u64, u64)>,
}

fn cells_of(b: &Body) -> Vec<Cell> {
    b.compartments.iter().filter_map(|l| Some((l.site.axis(0)?, l.site.axis(1)?, l.hue))).collect()
}

/// EVERY SINGLE CHANGE of a family's remainder, pair by pair, in the order
/// the eyes perceive the outputs. A pair whose frames differ is unread.
pub fn single_changes(family: &[(Grid, Grid)], remainders: &[Vec<(i64, i64, u8, u8)>]) -> Vec<Change> {
    let mut out = Vec::new();
    for (pair, ((x, y), rem)) in family.iter().zip(remainders).enumerate() {
        if x.len() != y.len() || x.iter().zip(y).any(|(a, b)| a.len() != b.len()) {
            continue;
        }
        let unsaid: Vec<(i64, i64)> = rem.iter().filter(|l| l.3 != 0).map(|l| (l.0, l.1)).collect();
        if unsaid.is_empty() {
            continue;
        }
        let ins = perceive(x);
        let in_cells: Vec<Vec<Cell>> = ins.iter().map(cells_of).collect();
        let in_moments: Vec<Moments> = in_cells.iter().map(|c| moments(c)).collect();
        for body in perceive(y) {
            let o = cells_of(&body);
            if !o.iter().any(|c| unsaid.contains(&(c.0, c.1))) {
                continue;
            }
            let standing: Vec<usize> = (0..ins.len()).filter(|&i| in_cells[i].iter().any(|c| o.iter().any(|d| d.0 == c.0 && d.1 == c.1))).collect();
            let [i] = standing.as_slice() else { continue };
            let m = &in_moments[*i];
            let mass_rank = in_moments.iter().fold((0u64, 0u64), |(a, b), n| (a + u64::from(n.mass > m.mass), b + u64::from(n.mass < m.mass)));
            let density_rank = m.density().map(|d| {
                in_moments.iter().filter_map(Moments::density).fold((0u64, 0u64), |(a, b), n| (a + u64::from(n > d), b + u64::from(n < d)))
            });
            out.push(Change {
                pair,
                difference: difference(&in_cells[*i], &o, y),
                output_moments: moments(&o),
                moments: m.clone(),
                shape: ins[*i].moduli.key().as_bytes().to_vec(),
                input: in_cells[*i].clone(),
                output: o,
                mass_rank,
                density_rank,
            });
        }
    }
    out
}

/// The keys a recolour is asked to be decided by, in the order [`decide`]
/// answers: none at all, the body's own volume, density, mass per hue,
/// shape, direction — and relational: its rank by mass and by density among
/// the bodies of its pair.
pub const KEYS: [&str; 8] = ["none", "volume", "density", "hue-mass", "shape", "direction", "mass rank", "density rank"];

/// WHAT ONE KEY DECIDED over a family's recolours in place, leave one out,
/// judged by replay: predicted and reproduced, refused (its others disagree),
/// alone (no other shares it), and whether every recolour reproduced.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Decision {
    pub predicted: u64,
    pub reproduced: u64,
    pub refused: u64,
    pub alone: u64,
    pub whole: bool,
}

fn judged<K: PartialEq>(keyed: Vec<(K, HueMap)>, readings: &[&Change]) -> Decision {
    let mut d = Decision::default();
    for (p, c) in decided(&keyed).into_iter().zip(readings) {
        match p {
            None => d.alone += 1,
            Some(None) => d.refused += 1,
            Some(Some(map)) => {
                d.predicted += 1;
                d.reproduced += u64::from(recolours_to(&map, &c.input, &c.output));
            }
        }
    }
    d.whole = !readings.is_empty() && d.reproduced == readings.len() as u64;
    d
}

/// A change's hue map: its recolours, each source hue once.
pub fn hue_map(c: &Change) -> HueMap {
    let mut map: HueMap = c.difference.recoloured.iter().map(|x| (x.2, x.3)).collect();
    map.sort_unstable();
    map.dedup();
    map
}

/// A RECOLOUR ITS OWN HUE MAP REPLAYS: every compartment of each source hue
/// goes to that one target. Where it does not, the change selects sites
/// within its body — no body-level key can decide it, and the decision's
/// ceiling is the count of these.
pub fn replayable(c: &Change) -> bool {
    recolours_to(&hue_map(c), &c.input, &c.output)
}

/// THE RECOLOURS IN PLACE among a family's single changes — kinematic, and
/// recolouring something — judged under every key of [`KEYS`].
pub fn decide(changes: &[Change]) -> [Decision; 8] {
    let rs: Vec<&Change> = changes.iter().filter(|c| c.difference.kinematic() && !c.difference.recoloured.is_empty()).collect();
    [
        judged(rs.iter().map(|c| ((), hue_map(c))).collect(), &rs),
        judged(rs.iter().map(|c| (c.moments.mass, hue_map(c))).collect(), &rs),
        judged(rs.iter().map(|c| (c.moments.density(), hue_map(c))).collect(), &rs),
        judged(rs.iter().map(|c| (c.moments.hues.clone(), hue_map(c))).collect(), &rs),
        judged(rs.iter().map(|c| (c.shape.clone(), hue_map(c))).collect(), &rs),
        judged(rs.iter().map(|c| (c.moments.direction(), hue_map(c))).collect(), &rs),
        judged(rs.iter().map(|c| (c.mass_rank, hue_map(c))).collect(), &rs),
        judged(rs.iter().map(|c| (c.density_rank, hue_map(c))).collect(), &rs),
    ]
}
