// 20, 51.3 — PERCEPTION: a grid becomes bodies, and two bodies become a
// vertex.
//
//! A body is a maximal hue-blind connected region on the three coordinates
//! of the base; its hues are the gauge it carries (70.4). `word_or_demand`
//! reads the versor that carries one body onto
//! another and the chain of hue transitions it leaves; `sensor_reads`
//! counts what the sensor was asked for.

use super::*;
use crate::transducer::base::{Site, FRAME_AXIS};

pub type Grid = Vec<Vec<u8>>;

/// A painted site of one frame's grid, the sensor's planar token:
/// `(row, col, hue)`.
pub(crate) type Painted = (i64, i64, u8);

/// A lamina of a body's support: a site of the base and its hue (51.3, 71).
pub type Lamina = crate::transducer::blade::Lamina;

/// A scene of painted sites as a grid, origin at the scene's min-corner.
/// The sensor embedding of one scene; not a ranking over candidates.
pub fn grid_of_painted(painted: &[(i64, i64, u8)]) -> Grid {
    // The head only SEEDS the bounding box; the box is folded over every
    // site below, so no member is chosen here.
    let [(r0, c0, _), ..] = painted else {
        return vec![vec![0]];
    };
    let (r0, c0) = (*r0, *c0);
    let (mut min_r, mut min_c, mut max_r, mut max_c) = (r0, c0, r0, c0);
    for &(r, c, _) in painted {
        if r < min_r {
            min_r = r;
        }
        if c < min_c {
            min_c = c;
        }
        if r > max_r {
            max_r = r;
        }
        if c > max_c {
            max_c = c;
        }
    }
    let h = (max_r - min_r + 1) as usize;
    let w = (max_c - min_c + 1) as usize;
    let mut g = vec![vec![0u8; w]; h];
    for &(r, c, hue) in painted {
        g[(r - min_r) as usize][(c - min_c) as usize] = hue;
    }
    g
}

/// THE SCENE A CHAIN DESCRIBES, as the two grids of one frame (50.6 seam
/// 1): the sites before and the sites after, both at the chain's own
/// min-corner, so the motion between them is the chain's and never an
/// artefact of two origins. `None` for an empty chain, which describes
/// no scene.
pub fn scene_of_chain(chain: &[(i64, i64, u8, u8)]) -> Option<(Grid, Grid)> {
    scene_at_of_chain(chain).map(|(_, before, after)| (before, after))
}

/// The same scene, with THE CORNER IT WAS FRAMED AT. A caller that has to
/// put a spoken site back where the chain had it needs the origin the
/// framing subtracted, and re-deriving that corner beside this reader is
/// the second carrier for one fact (law 12) — one of them would be the
/// one that drifts.
pub fn scene_at_of_chain(chain: &[(i64, i64, u8, u8)]) -> Option<((i64, i64), Grid, Grid)> {
    // As above: the head seeds the box the fold below measures.
    let [(r0, c0, _, _), ..] = chain else { return None };
    let (r0, c0) = (*r0, *c0);
    let (mut min_r, mut min_c, mut max_r, mut max_c) = (r0, c0, r0, c0);
    for &(r, c, _, _) in chain {
        if r < min_r {
            min_r = r;
        }
        if c < min_c {
            min_c = c;
        }
        if r > max_r {
            max_r = r;
        }
        if c > max_c {
            max_c = c;
        }
    }
    let h = (max_r - min_r + 1) as usize;
    let w = (max_c - min_c + 1) as usize;
    let mut before = vec![vec![0u8; w]; h];
    let mut after = vec![vec![0u8; w]; h];
    for &(r, c, b, a) in chain {
        before[(r - min_r) as usize][(c - min_c) as usize] = b;
        after[(r - min_r) as usize][(c - min_c) as usize] = a;
    }
    Some(((min_r, min_c), before, after))
}

#[cfg(test)]
pub(crate) fn q(v: i64) -> num_rational::BigRational {
    num_rational::BigRational::from_integer(num_bigint::BigInt::from(v))
}

/// A perceived body: its laminae — its support with hue on the three
/// coordinates of the base, ONE carrier (51.3) — its shape, and its
/// moduli.
#[derive(Clone, Debug)]
pub struct Body {
    pub laminae: Vec<Lamina>,
    pub shape: Shape,
    pub moduli: Invariant,
}

/// The body of one hue-blind connected component: its shape lifted from
/// its support, its moduli read off the shape, its laminae — and the hues
/// they carry — sorted. `None` where the moduli are not read at the
/// support's `n` (the topology past `n = 3` is 38.2's, BQ4): no chart is
/// invented for a body the harmonics cannot hear.
pub fn body_of(comp: Vec<Lamina>) -> Option<Body> {
    let shape = blade::shape_of(&sites_of(&comp));
    let moduli = Invariant(blade::moduli(&shape.blade)?);
    let mut laminae = comp;
    laminae.sort_unstable();
    laminae.dedup();
    Some(Body { laminae, shape, moduli })
}

/// The sites laminae stand at, sorted, each once.
pub(crate) fn sites_of(laminae: &[Lamina]) -> Vec<Site> {
    let mut s: Vec<Site> = laminae.iter().map(|l| l.site.clone()).collect();
    s.sort_unstable();
    s.dedup();
    s
}

thread_local! {
    static SENSOR_READS: Cell<u64> = const { Cell::new(0) };
    static UNHEARD: Cell<u64> = const { Cell::new(0) };
}

/// Components the perceivers found whose moduli are not read at their `n`
/// (`body_of`'s refusal), since the last take. Zero while intake reads
/// `n = 3`; the line that says so where it is not (BQ4).
pub fn take_unheard() -> u64 {
    UNHEARD.with(|c| c.replace(0))
}

/// The bodies of a perceiver's components, each refusal counted on
/// [`take_unheard`] rather than dropped unseen.
fn bodies_of(laminae: &[Lamina]) -> Vec<Body> {
    blade::components(laminae)
        .into_iter()
        .filter_map(|comp| {
            let body = body_of(comp);
            if body.is_none() {
                UNHEARD.with(|c| c.set(c.get() + 1));
            }
            body
        })
        .collect()
}

/// Grid sites visited by [`perceive`] since the last reset.
#[cfg(test)]
pub fn sensor_reads() -> u64 {
    SENSOR_READS.with(|c| c.get())
}

#[cfg(test)]
pub(crate) fn reset_sensor_reads() {
    SENSOR_READS.with(|c| c.set(0));
}

pub(crate) fn note_sensor_read() {
    SENSOR_READS.with(|c| c.set(c.get().saturating_add(1)));
}

/// Perceive one scene: its bodies, each a maximal hue-blind 4-connected
/// component of laminae standing in frame `0` (70.4); hue 0 is the
/// background the eye does not see. A body's colours are the gauge it
/// carries, and a seam between two of them is its own gauge section. Two
/// bodies meet at a corner, never along an edge — along an edge they would
/// be one body — and the form carries that corner as a contact face with
/// the edge transport of 50.3, so sliding and relative rotation are
/// measured on the edge. The court reads the percept as one sentence with
/// its body vertices and their edges, never as detached listings (50.1).
pub fn perceive(grid: &Grid) -> Vec<Body> {
    perceive_frames(std::slice::from_ref(grid))
}

/// Perceive one frame of a family as the sections of its bodies (51.4,
/// 52.5): the hue-blind 4-connected components in that frame, every
/// lamina standing at `k`. The section is the unit the pairing's fiber is
/// keyed by, because its chart is the chart speech and the court perceive
/// when they read a frame alone; a body stacked along `e₃` has a chart
/// that depends on its extent there, and a vertex keyed by it would find
/// nothing on a frame perceived by itself.
pub fn perceive_at(grid: &Grid, k: i64) -> Vec<Body> {
    let mut laminae: Vec<Lamina> = Vec::new();
    for (r, row) in grid.iter().enumerate() {
        for (c, &h) in row.iter().enumerate() {
            note_sensor_read();
            if h != 0 {
                laminae.push(Lamina::from((r as i64, c as i64, k, h)));
            }
        }
    }
    bodies_of(&laminae)
}

/// Perceive a family's frames as bodies on `(r, c, k)`: hue-blind
/// 6-connected components across the frames (51.3, 51.4, 70.4), the frame of each
/// grid its place in the family.
pub fn perceive_frames(grids: &[Grid]) -> Vec<Body> {
    let mut laminae: Vec<Lamina> = Vec::new();
    for (k, grid) in grids.iter().enumerate() {
        for (r, row) in grid.iter().enumerate() {
            for (c, &h) in row.iter().enumerate() {
                note_sensor_read();
                if h != 0 {
                    laminae.push(Lamina::from((r as i64, c as i64, k as i64, h)));
                }
            }
        }
    }
    bodies_of(&laminae)
}

/// THE CHAIN OF ONE SECTION between two bodies: each lamina of the source
/// carried by `v` and read on the image, with the hue it left and the hue
/// it landed on. The chain is the sensor's planar token (16.5); the frame
/// rides on the carriage. A body across frames puts two links on one site
/// of the section, which A5 refuses downstream — its chain is its sections,
/// one per frame (51.4).
pub(crate) fn chain_of(a: Option<&Body>, b: Option<&Body>, v: &Versor) -> Vec<(i64, i64, u8, u8)> {
    let mut chain = Vec::new();
    match (a, b) {
        (Some(src), Some(dst)) => {
            for l in &src.laminae {
                // A lamina the carriage lands nowhere in the image keeps
                // its own hue: nothing moved it.
                let h2 = landed_hue(v, &l.site, dst).map_or(l.hue, |found| found);
                if let Some((r, c)) = on_grid(&l.site) {
                    chain.push((r, c, l.hue, h2));
                }
            }
        }
        (None, Some(dst)) => {
            for l in &dst.laminae {
                if let Some((r, c)) = on_grid(&l.site) {
                    chain.push((r, c, 0, l.hue));
                }
            }
        }
        (Some(src), None) => {
            for l in &src.laminae {
                if let Some((r, c)) = on_grid(&l.site) {
                    chain.push((r, c, l.hue, 0));
                }
            }
        }
        (None, None) => {}
    }
    chain.sort_unstable();
    chain
}

/// The hue of the lamina of `dst` that `v` carries the lamina at `site`
/// onto: its centre carried and read back as a site. `None` where it lands
/// at infinity, off the base, or on no lamina of `dst`.
pub(crate) fn landed_hue(v: &Versor, site: &Site, dst: &Body) -> Option<u8> {
    let t = element::carry_site(v, site)?;
    dst.laminae.binary_search_by(|x| x.site.cmp(&t)).ok().map(|i| dst.laminae[i].hue)
}

/// A site's coordinates on the grid axes — the section's planar token
/// (16.5); every other axis rides on the pose. `None` for a site of a base
/// with fewer than the two grid axes, which has no section.
pub(crate) fn on_grid(s: &Site) -> Option<(i64, i64)> {
    let [ga, gb] = crate::transducer::base::GRID_AXES;
    Some((s.axis(ga)?, s.axis(gb)?))
}

pub(crate) fn hue_map_of_chain(chain: &[(i64, i64, u8, u8)]) -> Vec<(u8, u8)> {
    let mut seen: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
    for &(_, _, b, a) in chain {
        seen.insert(crate::transducer::element::hue_pair_key(b, a), ());
    }
    let mut m: Vec<(u8, u8)> = seen
        .scoped()
        .filter_map(|(k, _)| crate::transducer::element::hue_pair_of(&k))
        .collect();
    m.sort_unstable();
    m
}

/// What a demand vertex stands for: its kind, the chart it is listed under
/// and the links of the bodies it carries (50.5).
pub type Demanded = (crate::transducer::sentence::Demand, Invariant, Vec<Link>);

/// 50.5: a demand vertex over the bodies it stands for.
pub fn demand_vertex(kind: crate::transducer::sentence::Demand, chart: &Invariant, bodies: Vec<Link>) -> Vertex {
    Vertex::Demand { kind, chart: chart.clone(), bodies }
}

/// D3, DIRECTIVE 4 — THE MINT'S OWN VALVE LINE (68).
///
/// Every gate of [`oblique_demand`] at its own name, so a recut reads
/// WHERE the unspellable turns stop rather than only that none arrived.
/// `offered` is every pair no versor carried; the three refusals are the
/// three gates in the order they are passed; `minted` is what reached the
/// court. A starved valve and a closed one look identical on a total and
/// do not here.
#[derive(Default, Clone, Copy, Debug)]
pub struct DemandTally {
    /// Pairs reaching the mint: no versor carried them.
    pub offered: u64,
    /// Refused at gate 1 — `Mirrored` or `Incongruent`, proved permanent.
    pub permanent: u64,
    /// Refused at gate 2 — no arc: the pair orients only at the quartic,
    /// whose reading is `4φ` and whose witness would name a different
    /// rotation.
    pub arcless: u64,
    /// Refused at gate 3 — the arc names no prime the alphabet lacks.
    /// `z ~ ±i` is unspellable and still mints nothing.
    pub primeless: u64,
    /// `RotorPrime` demands minted, each with a primitive witness.
    pub minted: u64,
}

thread_local! {
    static DEMANDS: std::cell::Cell<DemandTally> =
        const { std::cell::Cell::new(DemandTally {
            offered: 0, permanent: 0, arcless: 0, primeless: 0, minted: 0,
        }) };
}

fn note_demand(f: impl Fn(&mut DemandTally)) {
    DEMANDS.with(|c| {
        let mut v = c.get();
        f(&mut v);
        c.set(v);
    });
}

/// Read the mint's tally and clear it.
pub fn take_demand_tally() -> DemandTally {
    DEMANDS.with(|c| c.replace(DemandTally::default()))
}

/// The mint's tally without clearing it, for a per-form line beside a
/// running total (law 12: only one reader may take).
pub fn demand_tally() -> DemandTally {
    DEMANDS.with(|c| c.get())
}

impl DemandTally {
    pub fn line(&self) -> String {
        format!(
            "offered {} | permanent {} arcless {} primeless {} | MINTED {}",
            self.offered, self.permanent, self.arcless, self.primeless, self.minted,
        )
    }
}

/// D3 — THE DEMAND AN UNSPELLABLE TURN LEAVES (68, ruled 2026-09-22).
///
/// Reached only where NO VERSOR carries `a` onto `b`. Most such pairs are
/// permanently unrelated and mint nothing; this asks the geometry which
/// kind it is and mints only for the one arm that is not proved permanent.
///
/// THE THREE GATES, AND EVERY ONE OF THEM CAN REFUSE:
///
/// 1. [`blade::NoFrame::Oblique`] and nothing else. `Mirrored` is
///    improper and `Incongruent` is the crystallographic restriction —
///    both permanent, and a demand minted from either would ask the court
///    to adjoin a generator for a relation that HAS no rotor. That is the
///    whole point of V1's split: before it, this arm held all three.
/// 2. A PRIMITIVE Gaussian arc. The court reads `π_p = gcd(s+t·i, p)` and
///    refuses a witness whose gcd is not of norm exactly `p`
///    (`REFUSE_LETTER_WITNESS`), so a composite arc mints letters the
///    court can only throw out — worse than minting none.
/// 3. A prime the alphabet does not already carry. `z ~ ±i` has norm 1
///    and names no prime at all: unspellable, and still not a
///    `RotorPrime`. It mints nothing rather than a demand with no prime
///    in it (rule 11).
///
/// THE PLANE IS THE SENSOR'S OWN. `deviatoric` reads `(Ixx−Iyy, 2Ixy)`,
/// which is `harmonic_on_plane((PLANE_ROW, PLANE_COL))` — the arc turns
/// in `e₁e₂` and the demand says so rather than defaulting.
fn oblique_demand(a: &Body, b: &Body, chart: &Invariant) -> Option<Demanded> {
    use crate::transducer::alphabet::{PLANE_COL, PLANE_ROW};
    use crate::transducer::blade;
    note_demand(|t| t.offered += 1);
    // THE PLANE IS THE SENSOR'S OWN, named once for the frame, the arc and
    // the demand (BQ4).
    let sensor = crate::transducer::base::GRID_PLANE;
    if blade::relative_rotor(&a.shape.blade, &b.shape.blade, sensor).err() != Some(blade::NoFrame::Oblique) {
        note_demand(|t| t.permanent += 1);
        return None;
    }
    let Some((s, t)) = blade::oblique_arc(&a.shape.blade, &b.shape.blade, sensor) else {
        note_demand(|t| t.arcless += 1);
        return None;
    };
    let plane = (PLANE_ROW, PLANE_COL);
    let norm = &s * &s + &t * &t;
    let Some(prime) = crate::transducer::alphabet::prime_outside_the_alphabet(&norm, plane) else {
        note_demand(|t| t.primeless += 1);
        return None;
    };
    note_demand(|t| t.minted += 1);
    // THE LINKS STAND UNMOVED, because nothing moved them. A chain is
    // each lamina carried by the versor and read on the image; there IS
    // no versor here, so the source's laminae keep their own hues and the
    // target's are read as they stand. Passing an identity versor instead
    // would write a carry the geometry just refused (checklist (m)).
    let mut bodies = chain_of(Some(a), None, &crate::transducer::element::identity_versor());
    bodies.extend(chain_of(None, Some(b), &crate::transducer::element::identity_versor()));
    bodies.sort_unstable();
    Some((
        crate::transducer::sentence::Demand::RotorPrime {
            plane,
            prime,
            witness: Box::new(crate::transducer::alphabet::arc_block(&s, &t)),
        },
        chart.clone(),
        bodies,
    ))
}

/// Carriage of two bodies spelled as a word form (50.2 F steps 1–2), or the
/// demand its rotor leaves: the versor measured, its rotor spelled by A22.
/// Class is the target's chart. `None` when no versor carries `a` onto `b`.
pub(crate) fn word_or_demand(a: &Body, b: &Body) -> Option<Result<Vertex, Demanded>> {
    word_or_demand_on_chart(a, b, &b.moduli)
}

/// The same reading on a NAMED CHART (52.5): the two bodies are the union
/// of a chart's fiber at one frame, whose own moduli are the union's and
/// not the chart the vertex acts on, so the chart is the caller's.
pub(crate) fn word_or_demand_on_chart(
    a: &Body,
    b: &Body,
    chart: &Invariant,
) -> Option<Result<Vertex, Demanded>> {
    let Some(car) = carriage(a, b) else {
        // D3 — A PAIR NO VERSOR CARRIES IS NOT NOTHING (68, ruled
        // 2026-09-22). This was a bare `?`: no versor, no vertex, no
        // demand, relation gone. That silent `None` is where 37.7's
        // pipeline starves — the court measured `Unspellable 0` and
        // `adjoin 0` because nothing was ever minted to reach it.
        //
        // An unspellable turn is CONTENT. It is asked for by name here
        // and minted only when the geometry proves it is one.
        return oblique_demand(a, b, chart).map(Err);
    };
    let chain = chain_of(Some(a), Some(b), &car.v);
    // A22 alone: this is the PERCEPT, which is what the memo holds. The
    // presentation enters afterwards, per mind (50.2 step 2).
    let derivation = match peel_versor(&car.v)? {
        Ok(w) => crate::transducer::alphabet::Derivation::spelled(w),
        Err(demand) => return Some(Err((demand, chart.clone(), chain))),
    };
    Some(Ok(word_vertex_on_chart(a, b, &car, derivation, chart)?))
}

/// THE WORD VERTEX OF A MEASURED CARRIAGE, given the derivation that
/// reaches it — the placement and dilation residues off the versor, the
/// gauge off the carriage or the chain, the address off the chain.
///
/// Split out because the derivation has TWO sources and the rest of the
/// vertex has one. A22's peel is the percept's; the ring's is a mind's,
/// found at 50.2 step 2 against a presentation the percept cannot hold.
/// Both build the same vertex around it.
pub(crate) fn word_vertex_with(
    a: &Body,
    b: &Body,
    car: &Carriage,
    derivation: crate::transducer::alphabet::Derivation,
) -> Option<Vertex> {
    word_vertex_on_chart(a, b, car, derivation, &b.moduli)
}

/// The same vertex acting on a NAMED CHART (52.5).
pub(crate) fn word_vertex_on_chart(
    a: &Body,
    b: &Body,
    car: &Carriage,
    derivation: crate::transducer::alphabet::Derivation,
    chart: &Invariant,
) -> Option<Vertex> {
    let chain = chain_of(Some(a), Some(b), &car.v);
    let t = element::translator_of(&car.v)?;
    let d = element::dilator_of(&car.v)?;
    let carriage = d.geo(&t);
    let gauge = if car.sigma.is_empty() {
        hue_map_of_chain(&chain)
    } else {
        car.sigma.clone()
    };
    Some(Vertex::word_on(
        derivation,
        chart.clone(),
        Box::new(carriage),
        gauge,
        None,
    ))
}

/// A body's laminae standing at frame `k`, or `None` where its laminae
/// stand in several frames: the sections `perceive_at` yields stand in one.
pub(crate) fn frame_of_body(b: &Body) -> Option<i64> {
    let mut at: Option<i64> = None;
    for l in &b.laminae {
        let k = l.site.axis(FRAME_AXIS)?;
        match at {
            None => at = Some(k),
            Some(f) if f == k => {}
            Some(_) => return None,
        }
    }
    at
}

/// The same laminae read in their frame's own coordinates, at `k = 0`:
/// the frame rides on the pose (16.5, 52.5), so a quotient measured here
/// is the one the vertex carries and the one a frame perceived alone reads.
/// A lamina with no frame axis stands in no frame and is carried as it is.
pub(crate) fn at_frame_zero(laminae: &[Lamina]) -> Vec<Lamina> {
    at_frame(laminae, 0)
}

/// Laminae moved to frame `k`, every other coordinate kept.
fn at_frame(laminae: &[Lamina], k: i64) -> Vec<Lamina> {
    laminae
        .iter()
        .map(|l| Lamina {
            site: match l.site.with_axis(FRAME_AXIS, k) {
                Some(s) => s,
                None => l.site.clone(),
            },
            hue: l.hue,
        })
        .collect()
}

/// A support read back at frame `k`, from laminae at frame zero.
pub(crate) fn support_at(laminae: &[Lamina], k: i64) -> Vec<Site> {
    sites_of(&at_frame(laminae, k))
}

/// The axes on which two charts differ, by position; an axis one chart
/// lacks differs.
pub(crate) fn differing_axes(a: &Invariant, b: &Invariant) -> Vec<u16> {
    let n = a.0.len().max(b.0.len());
    (0..n).filter(|&i| a.0.get(i) != b.0.get(i)).map(|i| i as u16).collect()
}

/// WHY A PAIR CARRIES NO EDGE (50.3, 13.9): no single versor moves the two
/// bodies rigidly; a versor with no rotor, translator or dilator reading —
/// the `Cl(3,1)` bridge (51.2); or a stabilizer element whose rotor the
/// seventeen do not spell — 37.7's candidate, carried as the demand the
/// peel typed (50.4) and not an edge invented for it (rule 11).
#[derive(Debug)]
pub(crate) enum EdgeRefusal {
    NotRigid,
    NoReading,
    Unspelled(crate::transducer::sentence::Demand),
}

/// THE EDGES A PAIR FORM ASKED FOR, BY WHAT CAME BACK (§8.1, measurement
/// only): measured, or refused for one of `EdgeRefusal`'s three reasons.
/// Read on the `pair form` line, so the corpus says whether any
/// stabilizer element on it is 37.7's candidate.
#[derive(Default)]
pub(crate) struct EdgeTally {
    pub(crate) measured: usize,
    pub(crate) unspelled: usize,
    pub(crate) unread: usize,
    pub(crate) not_rigid: usize,
}

impl EdgeTally {
    /// Count one asked edge and hand the caller what it carries.
    /// EVERY MEMBER'S REFUSAL IS COUNTED, not the first. The stabilizer the
    /// coset quotients is exactly the freedom in which member to read `Γ`
    /// through, so each member refuses for its own reason — and an
    /// `Unspelled` standing behind another member's `NotRigid` was 37.7's
    /// candidate, discarded for sorting second. The refusals arrive in the
    /// coset's own content-fixed order and are only ever iterated, never
    /// indexed.
    ///
    /// The counters are therefore per READING and may exceed the number of
    /// pairs asked. That is what they now count.
    pub(crate) fn take(
        &mut self,
        asked: Option<Result<crate::transducer::sentence::Transport, Vec<EdgeRefusal>>>,
    ) -> Option<crate::transducer::sentence::Transport> {
        match asked {
            Some(Ok(t)) => {
                self.measured += 1;
                Some(t)
            }
            Some(Err(refusals)) => {
                for refusal in refusals {
                    match refusal {
                        // 37.7's candidate at an edge, said with the kind
                        // the peel typed: the one line on this path that
                        // names a generator the language lacks.
                        EdgeRefusal::Unspelled(d) => {
                            self.unspelled += 1;
                            eprintln!("  edge unspelled: {}", d.label());
                        }
                        EdgeRefusal::NoReading => self.unread += 1,
                        EdgeRefusal::NotRigid => self.not_rigid += 1,
                    }
                }
                None
            }
            None => None,
        }
    }
}

/// THE EDGE A TRANSITION `Γ` IS (50.3): its rotor spelled by A22's peel as
/// the word, its placement and dilation as the carriage, and no gauge —
/// the caller reads the hue transition where it has one.
pub(crate) fn edge_of(gamma: &Versor) -> Result<crate::transducer::sentence::Transport, EdgeRefusal> {
    let word = match peel_versor(gamma) {
        None => return Err(EdgeRefusal::NoReading),
        Some(Err(d)) => return Err(EdgeRefusal::Unspelled(d)),
        Some(Ok(w)) => w,
    };
    // 54.7 R3 — THE EDGE IS A CONNECTION, SO ITS DISPLACEMENT IS A
    // TRANSVECTION (ruled 2026-09-21).
    //
    // This read `translator_of(gamma)`: the conformal motor
    // `T(d) = 1 − ½ d e_∞`, whose generator is NULL because
    // `e_∞² = e₋² + e₊² = 0`. With `R_a = R_b` the relative transition
    // `V_b ∘ V_a⁻¹` collapses to `R · T(d_b − d_a) · R⁻¹`, a conjugated
    // parabolic — so the edge carried `I₁ = 0` by construction, and then
    // `Δ = −I₂² < 0` forbids any rational eigenplane split. Four corpus
    // cycles read Parabolic AND Undecomposed for that one reason, and they
    // are the same four seen from two directions.
    //
    // MEASURED, before the change: dropping the member carriage from the
    // cycle composite left those four completely unchanged, which is what
    // proves the null generator was sealed in HERE rather than accumulated
    // from poses around the loop.
    //
    // 51.2 holds both readings in one algebra: a lattice POSE keeps the
    // conformal translator, which commutes and is derived; a CONNECTION
    // takes the geodesic transvection `eᵢ ∧ e₋`, which does not commute —
    // and the failure to commute is the curvature this edge exists to
    // carry.
    let t = element::transvection_of(gamma).ok_or(EdgeRefusal::NoReading)?;
    let d = element::dilator_of(gamma).ok_or(EdgeRefusal::NoReading)?;
    Ok(crate::transducer::sentence::Transport { word, carriage: d.geo(&t), gauge: Vec::new() })
}

/// THE EDGE TRANSPORT of 50.3 between two carried bodies. `V_ij` carries
/// the pair-body `(a_i ∪ a_j)` onto `(b_i ∪ b_j)`; it exists iff the pair
/// moved rigidly, and then `Γ_ij = V_i⁻¹ · V_ij` lies in the stabilizer of
/// `a_i` or the exchange of the two bodies. A refusal is a remainder edge,
/// and says which (`EdgeRefusal`).
///
/// EVERY MEMBER OF THE COSET IS A CANDIDATE, geodesic first. It read
/// `versor_between` — the geodesic alone — and the whole pair refused when
/// that one member's `Γ` had no reading, though the stabilizer the coset
/// quotients is exactly the freedom in which member to read `Γ` through.
/// The first member that reads answers, in
/// [`crate::transducer::blade::versor_coset_ordered`]'s content-fixed
/// order, so the answer does not depend on the order the coset was
/// measured in; the geodesic's refusal is the one reported when none
/// reads, because that is the member the naming uses.
pub(crate) fn edge_transport(
    v_i: &Versor,
    a_i: &Body,
    a_j: &Body,
    b_i: &Body,
    b_j: &Body,
) -> Result<crate::transducer::sentence::Transport, Vec<EdgeRefusal>> {
    let union = |x: &Body, y: &Body| -> Shape {
        let laminae: Vec<Lamina> = x.laminae.iter().chain(y.laminae.iter()).cloned().collect();
        blade::shape_of(&sites_of(&laminae))
    };
    let members = blade::versor_coset_ordered(&union(a_i, a_j), &union(b_i, b_j));
    let inv = v_i.versor_inverse().ok_or_else(|| vec![EdgeRefusal::NoReading])?;
    let mut refusals: Vec<EdgeRefusal> = Vec::new();
    for v_ij in &members {
        match edge_of(&inv.geo(v_ij)) {
            Ok(transport) => return Ok(transport),
            Err(e) => refusals.push(e),
        }
    }
    if refusals.is_empty() {
        // Nothing refused because nothing carried: the empty coset, which
        // is the pair not moving rigidly. Not a default standing in for a
        // reading (rule 11).
        refusals.push(EdgeRefusal::NotRigid);
    }
    Err(refusals)
}

/// A body's support on the sites of the base, sorted.
pub(crate) fn support_of(body: &Body) -> Vec<Site> {
    sites_of(&body.laminae)
}

// `ring_of_support` STOOD HERE AND IS DELETED — T1 (60.5, ruled
// 2026-09-22). It dilated a support by a Manhattan 4-neighbourhood on ℤ²
// and had exactly two callers: the cover's meet index, purged by 58.3, and
// `star_and_site_faces`, purged with this. Between them they were the
// engine's entire notion of incidence, and both were reading the dilation
// rather than the corpus — 184 of 186 meets, and every `b₁`,
// `loop_defects` and spectrum taken through the form's faces.
//
// It is deleted rather than left unused so that combinatorial pixel
// inflation cannot silently return: a stencil with no caller is a stencil
// waiting for one. Contact between disjoint bodies is the conformal strain
// of 59.3, condensed at the mind's own horizon (59.6), and it enters
// through the tether or it does not enter at all.

pub(crate) fn frame_of(g: &Grid) -> (usize, usize) {
    (g.len(), crate::arc::grid_width(g))
}

/// Does everything a form spoke lie inside the grid it was spoken against?
/// The test extraction's step 3 decides a glue by, and the same test a
/// voice answers a demand by (L8q), so the two organs read one thing.
pub fn painted_within_grid(painted: &[Painted], g: &Grid) -> bool {
    painted.iter().all(|&(r, c, h)| {
        g.get(r as usize)
            .and_then(|row| row.get(c as usize))
            .copied()
            == Some(h)
    })
}

/// The scene's bodies indexed by their own chart (Y4, 24.4, 20).
/// A chart whose fiber holds exactly one body is "the unique one", read as
/// `fiber.iter().nth(1).is_none()` (W10), without a count compared to a literal.
pub fn bodies_by_chart(bodies: &[Body]) -> crate::network::nerve::Keyed<Vec<&Body>> {
    let mut fibers: crate::network::nerve::Keyed<Vec<&Body>> = crate::network::nerve::Keyed::new();
    for b in bodies {
        let k = b.moduli.key();
        match fibers.get_mut(&k) {
            Some(v) => v.push(b),
            None => {
                fibers.insert(k, vec![b]);
            }
        }
    }
    fibers
}

/// The chart whose fiber in the scene holds exactly one body (Y4, 24.4, 21 step 1, W10).
/// Answers "the chart whose fiber holds one" without a count compared to a literal.
pub fn chart_holding_one(bodies: &[Body]) -> Option<crate::transducer::stalk::Invariant> {
    let fibers = bodies_by_chart(bodies);
    let mut singletons = fibers
        .scoped()
        .filter_map(|(_, fiber)| {
            let mut it = fiber.iter();
            let first = it.next()?;
            if it.next().is_none() {
                Some(first.moduli.clone())
            } else {
                None
            }
        });
    let unique = singletons.next()?;
    if singletons.next().is_none() {
        Some(unique)
    } else {
        None
    }
}
