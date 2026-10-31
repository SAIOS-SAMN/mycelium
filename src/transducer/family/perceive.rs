// 20, 51.3 — PERCEPTION: a grid becomes bodies, and two bodies become a
// vertex.
//
//! A body is a maximal hue-blind connected region on the three coordinates
//! of the base; its hues are the gauge it carries (70.4). `word_or_demand`
//! reads the versor that carries one body onto
//! another and the chain of hue transitions it leaves.

use super::*;
use crate::transducer::base::{Site, FRAME_AXIS};

pub type Grid = Vec<Vec<u8>>;

/// A painted site of one frame's grid, the sensor's planar token:
/// `(row, col, hue)`.
pub(crate) type Painted = (i64, i64, u8);

/// A compartment of a body's support: a site of the base and its hue (51.3, 71).
pub type Compartment = crate::transducer::blade::Compartment;

/// A scene of painted sites as a grid, origin at the scene's min-corner.
/// The sensor embedding of one scene; not a ranking over candidates.
pub fn grid_of_painted(painted: &[(i64, i64, u8)]) -> Grid {
    // The head only SEEDS the bounding box; the box is folded over every
    // site below, so no member is chosen here.
    let void = crate::arc::ground();
    let [(r0, c0, _), ..] = painted else {
        return vec![vec![void]];
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
    #[expect(clippy::cast_possible_truncation, reason = "STD1 C1 — the sensor's: the painted sites' bounding-box height, an i64 span ≥ 1 of the sites the sensor delivers, narrowing only on a 32-bit target")]
    let h = (max_r - min_r + 1) as usize;
    #[expect(clippy::cast_possible_truncation, reason = "STD1 C1 — the sensor's: the painted sites' bounding-box width, an i64 span ≥ 1 of the sites the sensor delivers, narrowing only on a 32-bit target")]
    let w = (max_c - min_c + 1) as usize;
    let mut g = vec![vec![void; w]; h];
    #[expect(clippy::cast_possible_truncation, reason = "STD1 C1 — the sensor's: a painted site's offset inside the bounding box just sized from these same sites, 0 ≤ r − min_r < h and 0 ≤ c − min_c < w")]
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
    #[expect(clippy::cast_possible_truncation, reason = "STD1 C1 — the sensor's: the chain's bounding-box height, an i64 span ≥ 1 of the sites the sensor delivers, narrowing only on a 32-bit target")]
    let h = (max_r - min_r + 1) as usize;
    #[expect(clippy::cast_possible_truncation, reason = "STD1 C1 — the sensor's: the chain's bounding-box width, an i64 span ≥ 1 of the sites the sensor delivers, narrowing only on a 32-bit target")]
    let w = (max_c - min_c + 1) as usize;
    let void = crate::arc::ground();
    let mut before = vec![vec![void; w]; h];
    let mut after = vec![vec![void; w]; h];
    #[expect(clippy::cast_possible_truncation, reason = "STD1 C1 — the sensor's: a chain site's offset inside the bounding box just sized from these same sites, 0 ≤ r − min_r < h and 0 ≤ c − min_c < w")]
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

/// A perceived body: its compartments — its support with hue on the three
/// coordinates of the base, ONE carrier (51.3) — its shape, and its
/// moduli.
#[derive(Clone, Debug)]
pub struct Body {
    pub compartments: Vec<Compartment>,
    pub shape: Shape,
    pub moduli: Invariant,
}

/// The body of one hue-blind connected component: its shape lifted from
/// its support, its moduli read off the shape, its compartments — and the hues
/// they carry — sorted. `None` where the moduli are not read at the
/// support's `n` (the topology past `n = 3` is 38.2's, BQ4): no chart is
/// invented for a body the harmonics cannot hear.
pub fn body_of(comp: Vec<Compartment>) -> Option<Body> {
    let shape = blade::shape_of(&sites_of(&comp));
    let moduli = Invariant(blade::moduli(&shape.blade)?);
    let mut compartments = comp;
    compartments.sort_unstable();
    compartments.dedup();
    Some(Body { compartments, shape, moduli })
}

/// The sites compartments stand at, sorted, each once.
pub(crate) fn sites_of(compartments: &[Compartment]) -> Vec<Site> {
    let mut s: Vec<Site> = compartments.iter().map(|l| l.site.clone()).collect();
    s.sort_unstable();
    s.dedup();
    s
}

/// The bodies of a perceiver's components; a refused one is left out.
fn bodies_of(compartments: &[Compartment]) -> Vec<Body> {
    blade::components(compartments).into_iter().filter_map(body_of).collect()
}

/// Perceive one scene: its bodies, each a maximal hue-blind 4-connected
/// component of compartments standing in frame `0` (70.4); the scene's
/// ground (`arc::ground`: hue 0, or the hue its family is drawn on) is the
/// background the eye does not see. A body's colours are the gauge it
/// carries, and a seam between two of them is its own gauge section. Two
/// bodies meet at a corner, never along an edge — along an edge they would
/// be one body — and the form carries that corner as a contact face with
/// the edge transport of 50.3, so sliding and relative rotation are
/// measured on the edge. The court reads the percept as one sentence with
/// its body vertices and the vertices' edges, never as detached listings (50.1).
pub fn perceive(grid: &Grid) -> Vec<Body> {
    perceive_frames(std::slice::from_ref(grid))
}

/// Perceive one frame of a family as the sections of its bodies (51.4,
/// 52.5): the hue-blind 4-connected components in that frame, every
/// compartment standing at `k`. The section is the unit the pairing's fibre is
/// keyed by, because its chart is the chart speech and the court perceive
/// when they read a frame alone; a body stacked along `e₃` has a chart
/// that depends on its extent there, and a vertex keyed by it would find
/// nothing on a frame perceived by itself.
pub fn perceive_at(grid: &Grid, k: i64) -> Vec<Body> {
    let void = crate::arc::ground();
    let mut compartments: Vec<Compartment> = Vec::new();
    for (r, row) in grid.iter().enumerate() {
        for (c, &h) in row.iter().enumerate() {
            if h != void {
                compartments.push(Compartment::from((r as i64, c as i64, k, h)));
            }
        }
    }
    bodies_of(&compartments)
}

/// 78.167 Phase A — THE SCENE'S MONOCHROME PARTS: each hue's 4-connected
/// runs of painted cells, every one a body of its own (a hue-blind body of
/// several hues is several parts; a body of one hue is one part). What a
/// clause on parts selects among (`Act::Parts`): a body recoloured in part is
/// parts recoloured whole. Read in the lattice's order, hue by hue.
pub fn perceive_parts(grid: &Grid) -> Vec<Body> {
    let void = crate::arc::ground();
    let mut by_hue: crate::network::nerve::Keyed<Vec<Compartment>> = crate::network::nerve::Keyed::new();
    for (r, row) in grid.iter().enumerate() {
        for (c, &h) in row.iter().enumerate() {
            if h == void {
                continue;
            }
            let l = Compartment::from((r as i64, c as i64, 0, h));
            match by_hue.get_mut(&[h]) {
                Some(ls) => ls.push(l),
                None => {
                    by_hue.insert([h], vec![l]);
                }
            }
        }
    }
    by_hue.scoped().flat_map(|(_, ls)| bodies_of(ls)).collect()
}

/// A FAMILY'S FRAMES, PERCEIVED: each pair's input bodies and output bodies,
/// frame `k` at `k`.
pub type Frames = Vec<(Vec<Body>, Vec<Body>)>;

/// THE FAMILY'S FRAMES PERCEIVED ONCE (rule 10, ruled 2026-09-26): each
/// frame's bodies read by its components, the one reading an extraction's
/// stroke is sized by ([`super::extraction_spend`]) and the percept then
/// reads, so no frame's components are read a second time to size the draw.
pub fn perceive_family(family: &[(Grid, Grid)]) -> Frames {
    crate::arc::on_ground_of(family, || {
        family
            .iter()
            .enumerate()
            .map(|(k, (a, b))| (perceive_at(a, k as i64), perceive_at(b, k as i64)))
            .collect()
    })
}

/// Perceive a family's frames as bodies on `(r, c, k)`: hue-blind
/// 6-connected components across the frames (51.3, 51.4, 70.4), the frame of each
/// grid its place in the family.
pub fn perceive_frames(grids: &[Grid]) -> Vec<Body> {
    let void = crate::arc::ground();
    let mut compartments: Vec<Compartment> = Vec::new();
    for (k, grid) in grids.iter().enumerate() {
        for (r, row) in grid.iter().enumerate() {
            for (c, &h) in row.iter().enumerate() {
                if h != void {
                    compartments.push(Compartment::from((r as i64, c as i64, k as i64, h)));
                }
            }
        }
    }
    bodies_of(&compartments)
}

/// THE CHAIN OF ONE SECTION between two bodies: each compartment of the source
/// carried by `v` and read on the image, with the hue it left and the hue
/// it landed on. The chain is the sensor's planar token (16.5); the frame
/// rides on the carriage. A body across frames puts two links on one site
/// of the section, which A5 refuses downstream — its chain is its sections,
/// one per frame (51.4).
pub(crate) fn chain_of(a: Option<&Body>, b: Option<&Body>, v: &Versor) -> Vec<(i64, i64, u8, u8)> {
    let void = crate::arc::ground();
    let mut chain = Vec::new();
    match (a, b) {
        (Some(src), Some(dst)) => {
            for l in &src.compartments {
                // A compartment the carriage lands nowhere in the image keeps
                // its own hue: nothing moved it.
                let h2 = landed_hue(v, &l.site, dst).map_or(l.hue, |found| found);
                if let Some((r, c)) = on_grid(&l.site) {
                    chain.push((r, c, l.hue, h2));
                }
            }
        }
        (None, Some(dst)) => {
            for l in &dst.compartments {
                if let Some((r, c)) = on_grid(&l.site) {
                    chain.push((r, c, void, l.hue));
                }
            }
        }
        (Some(src), None) => {
            for l in &src.compartments {
                if let Some((r, c)) = on_grid(&l.site) {
                    chain.push((r, c, l.hue, void));
                }
            }
        }
        (None, None) => {}
    }
    chain.sort_unstable();
    chain
}

/// The hue of the compartment of `dst` that `v` carries the compartment at `site`
/// onto: its centre carried and read back as a site. `None` where it lands
/// at infinity, off the base, or on no compartment of `dst`.
pub(crate) fn landed_hue(v: &Versor, site: &Site, dst: &Body) -> Option<u8> {
    let t = element::carry_site(v, site)?;
    dst.compartments.binary_search_by(|x| x.site.cmp(&t)).ok().map(|i| dst.compartments[i].hue)
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
    Vertex::demand(kind, chart.clone(), bodies)
}

/// CV1b F3 — THE POLE'S DEMANDS STAND AS THE SENTENCE'S OWN VERTICES, one
/// per demand's own coordinates (its prime and its witness, the plane with
/// them): a gap reached on the word path and at the pole stands once. A
/// demand vertex carries no support and no face, so standing it after the
/// faces is the vertex it would have been before them; the form's key is its
/// vertices' coordinates, never the vertices' order.
pub(crate) fn stand_demands(vertices: &mut crate::transducer::sentence::Vertices, demanded: Vec<Demanded>) {
    use crate::transducer::sentence::Demand;
    let coords = |kind: &Demand| -> Option<Vec<u8>> {
        let Demand::RotorPrime { .. } = kind else { return None };
        let mut key = Vec::new();
        kind.to_bytes(&mut key);
        Some(key)
    };
    let mut standing: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
    for (_, v) in vertices.entries() {
        if let Some(k) = match v {
            Vertex::Demand { kind, .. } => coords(kind),
            _ => None,
        } {
            standing.insert(&k, ());
        }
    }
    // A kind with no coordinates here, a demand its own type refuses, and
    // one already standing do not stand again.
    for (kind, chart, mut links) in demanded {
        let Some(k) = coords(&kind) else {
            continue;
        };
        if kind.typed().is_err() || standing.contains(&k) {
            continue;
        }
        standing.insert(&k, ());
        links.sort_unstable();
        links.dedup();
        let v = demand_vertex(kind, &chart, links);
        if let Some(at) = v.at() {
            vertices.put(&at, v);
        }
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
/// 1. [`blade::NoPose::Oblique`] and nothing else. `Mirrored` is
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
/// in `e₁e₂` and the demand says so rather than defaulting. It stays the
/// sensor's after FR1: `Oblique` is the square base's question and the
/// witness a Gaussian arc in that plane, so the pose over the tube
/// ([`blade::relative_pose`]) does not replace it here.
fn oblique_demand(a: &Body, b: &Body, chart: &Invariant, budget: &mut crate::physics::Budget) -> Option<Demanded> {
    oblique_demanded(a, b, (&a.shape.blade, &b.shape.blade), chart, budget)
}

/// D3's DEMAND ON ONE PAIR OF BODIES, the bodies' blades given: the three gates
/// (`oblique_rotor_prime`), then the pair's own compartments as the demand's
/// links. The word path and the contact's pole (CV1b F3) read it alike.
pub(crate) fn oblique_demanded(
    a: &Body,
    b: &Body,
    (blade_a, blade_b): (&crate::transducer::blade::Blade, &crate::transducer::blade::Blade),
    chart: &Invariant,
    budget: &mut crate::physics::Budget,
) -> Option<Demanded> {
    let demand = oblique_rotor_prime(blade_a, blade_b, budget)?;
    // THE LINKS STAND UNMOVED, because nothing moved them. A chain is
    // each compartment carried by the versor and read on the image; there IS
    // no versor here, so the source's compartments each keeps its own hue and the
    // target's are read as they stand. Passing an identity versor instead
    // would write a carry the geometry just refused (checklist (m)).
    let mut bodies = chain_of(Some(a), None, &crate::transducer::element::identity_versor());
    bodies.extend(chain_of(None, Some(b), &crate::transducer::element::identity_versor()));
    bodies.sort_unstable();
    Some((demand, chart.clone(), bodies))
}

/// D3's THREE GATES, ONE STATEMENT (law 12): the pair reads exactly
/// `NoPose::Oblique` on the sensor's plane, its Gaussian arc is primitive,
/// and its prime lies outside the alphabet — or the mint's valve line says
/// which gate stopped it. The `RotorPrime` with its witness, `arc_block`.
pub(crate) fn oblique_rotor_prime(
    blade_a: &crate::transducer::blade::Blade,
    blade_b: &crate::transducer::blade::Blade,
    budget: &mut crate::physics::Budget,
) -> Option<crate::transducer::sentence::Demand> {
    use crate::transducer::alphabet::{PLANE_COL, PLANE_ROW};
    use crate::transducer::blade;
    // THE PLANE IS THE SENSOR'S OWN, named once for the frame, the arc and
    // the demand (BQ4).
    let sensor = crate::transducer::base::GRID_PLANE;
    // GATE 1: only the oblique arm passes. A turn is no refusal; a held
    // stabilizer is no refusal; any other answer is a refusal.
    let Err(blade::NoPose::Oblique) = blade::relative_rotor(blade_a, blade_b, sensor, budget) else {
        return None;
    };
    let (s, t) = blade::oblique_arc(blade_a, blade_b, sensor)?;
    let plane = (PLANE_ROW, PLANE_COL);
    let norm = &s * &s + &t * &t;
    let prime = crate::transducer::alphabet::prime_outside_the_alphabet(&norm, plane)?;
    Some(crate::transducer::sentence::Demand::RotorPrime {
        plane,
        prime,
        witness: Box::new(crate::transducer::alphabet::arc_block(&s, &t)),
    })
}

/// Carriage of two bodies spelled as a word form (50.2 F steps 1–2), or the
/// demand its rotor leaves: the versor measured, its rotor spelled by A22.
/// Class is the target's chart. `None` when no versor carries `a` onto `b`.
pub(crate) fn word_or_demand(
    a: &Body,
    b: &Body,
    budget: &mut crate::physics::Budget,
) -> Option<Result<Vertex, Demanded>> {
    word_or_demand_on_chart(a, b, &b.moduli, budget)
}

/// The same reading on a NAMED CHART (52.5): the two bodies are the union
/// of a chart's fibre at one frame, whose own moduli are the union's and
/// not the chart the vertex acts on, so the chart is the caller's.
pub(crate) fn word_or_demand_on_chart(
    a: &Body,
    b: &Body,
    chart: &Invariant,
    budget: &mut crate::physics::Budget,
) -> Option<Result<Vertex, Demanded>> {
    let Some(car) = carriage(a, b, budget) else {
        // D3 — A PAIR NO VERSOR CARRIES IS NOT NOTHING (68, ruled
        // 2026-09-22). This was a bare `?`: no versor, no vertex, no
        // demand, relation gone. That silent `None` is where 37.7's
        // pipeline starves — the court measured `Unspellable 0` and
        // `adjoin 0` because nothing was ever minted to reach it.
        //
        // An unspellable turn is CONTENT. It is asked for by name here
        // and minted only when the geometry proves it is one.
        return oblique_demand(a, b, chart, budget).map(Err);
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
        // The frames the member holds are the percept's to write, once the
        // frames reading this versor are grouped (AT1).
        Vec::new(),
        None,
    ))
}

/// A body's compartments standing at frame `k`, or `None` where its compartments
/// stand in several frames: the sections `perceive_at` yields stand in one.
pub(crate) fn frame_of_body(b: &Body) -> Option<i64> {
    let mut at: Option<i64> = None;
    for l in &b.compartments {
        let k = l.site.axis(FRAME_AXIS)?;
        match at {
            None => at = Some(k),
            Some(f) if f == k => {}
            Some(_) => return None,
        }
    }
    at
}

/// The same compartments read in the frame's own coordinates, at `k = 0`:
/// the frame rides on the pose (16.5, 52.5), so a quotient measured here
/// is the one the vertex carries and the one a frame perceived alone reads.
/// A compartment with no frame axis stands in no frame and is carried as it is.
pub(crate) fn at_frame_zero(compartments: &[Compartment]) -> Vec<Compartment> {
    at_frame(compartments, 0)
}

/// Compartments moved to frame `k`, every other coordinate kept.
fn at_frame(compartments: &[Compartment], k: i64) -> Vec<Compartment> {
    compartments
        .iter()
        .map(|l| Compartment {
            site: match l.site.with_axis(FRAME_AXIS, k) {
                Some(s) => s,
                None => l.site.clone(),
            },
            hue: l.hue,
        })
        .collect()
}

/// A support read back at frame `k`, from compartments at frame zero.
pub(crate) fn support_at(compartments: &[Compartment], k: i64) -> Vec<Site> {
    sites_of(&at_frame(compartments, k))
}

/// The axes on which two charts differ, by position; an axis one chart
/// lacks differs.
#[expect(clippy::cast_possible_truncation, reason = "AT1 debt — a slot number of the form's Vec (an axis position in the longer of two charts' Invariant vectors); the atlas row replaces the slot by the chart's coordinates")]
pub(crate) fn differing_axes(a: &Invariant, b: &Invariant) -> Vec<u16> {
    let n = a.0.len().max(b.0.len());
    (0..n).filter(|&i| a.0.get(i) != b.0.get(i)).map(|i| i as u16).collect()
}

/// WHY A PAIR CARRIES NO EDGE (50.3, 13.9): a body no conformal motion
/// carries — a tear (78.103: a pair that merely moved differentially is
/// related by its relative motion, never refused); a versor with no rotor, translator or dilator reading —
/// the `Cl(3,1)` bridge (51.2); or a stabilizer element whose rotor the
/// seventeen do not spell — 37.7's candidate, carried as the demand the
/// peel typed (50.4) and not an edge invented for it (rule 11).
#[derive(Debug)]
pub(crate) enum EdgeRefusal {
    NotRigid,
    NoReading,
    Unspelled(
        #[expect(
            dead_code,
            reason = "78.16 C3 — the typed demand at an unspelled edge reached only a printed line; no mind or court consumes it yet (reported, law 7: kept, never discarded)"
        )]
        crate::transducer::sentence::Demand,
    ),
}

/// THE EDGES A PAIR FORM ASKED FOR, BY WHAT CAME BACK (§8.1, measurement
/// only): measured, or refused for one of `EdgeRefusal`'s three reasons.
/// Read on the `pair form` line, so the corpus says whether any
/// stabilizer element on it is 37.7's candidate.
#[derive(Default)]
pub(crate) struct EdgeTally {
    pub(crate) measured: usize,
    /// 78.103 — of the measured, the edges that are a RELATIVE MOTION: two
    /// bodies that moved differently, `Γ = ṽᵢ·vⱼ`, no versor carrying the
    /// bodies' union.
    pub(crate) relative: usize,
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
                        // 37.7's candidate at an edge: a generator the
                        // language lacks, counted.
                        EdgeRefusal::Unspelled(_) => self.unspelled += 1,
                        EdgeRefusal::NoReading => self.unread += 1,
                        EdgeRefusal::NotRigid => self.not_rigid += 1,
                    }
                }
                None
            }
            None => None,
        }
    }

    /// One edge asked of [`edge_transport`], counted as [`Self::take`]
    /// counts it, a relative motion counted apart (78.103).
    pub(crate) fn take_carried(
        &mut self,
        asked: Result<(crate::transducer::sentence::Transport, Carried), Vec<EdgeRefusal>>,
    ) -> Option<crate::transducer::sentence::Transport> {
        if matches!(asked, Ok((_, Carried::Relative))) {
            self.relative += 1;
        }
        self.take(Some(asked.map(|(t, _)| t)))
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
    // from poses around the orbit.
    //
    // 51.2 holds both readings in one algebra: a lattice POSE keeps the
    // conformal translator, which commutes and is derived; a CONNECTION
    // takes the geodesic transvection `eᵢ ∧ e₋`, which does not commute —
    // and the failure to commute is the curvature this edge exists to
    // carry.
    //
    // 78.97 — SUPERSEDED BY THE LAYERING (ruled 2026-10-04: what
    // is seen are rays through the luminal). An edge is what was SEEN: a body carried
    // from one placement to the next, a ray on the luminal cone, and its
    // displacement's own generator is null — the conformal translator
    // `T(d)`, never a boost. A boost never returns, so every ring of seen
    // edges read unbound; the "parabolic" cycles 54.7 refused are what a
    // seen ray IS, composed exactly by 78.75's flow law (`k = B² = 0`): a
    // ring of rays closes flat, or extrapolates as a nilpotent theorem. The
    // curvature an edge carries is the turn its word holds and the
    // non-commuting of rays around a ring — never a hyperbolic slip.
    let t = element::translator_of(gamma).ok_or(EdgeRefusal::NoReading)?;
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
///
/// 78.103 — TWO BODIES THAT MOVED DIFFERENTIALLY ARE RELATED BY THE
/// RELATIVE MOTION (78.63 governs 50.3). Where no versor carries the union,
/// the pair did not move rigidly — a symptom, never a breach — and the edge
/// is `Γ = ṽᵢ·vⱼ`, the one body's motion read from the other's, a bend like
/// any other with its own modes (turn, null slide, boost). Only a body no
/// conformal motion carries is a TEAR, and stays `NotRigid`.
pub(crate) fn edge_transport(
    (v_i, v_j): (&Versor, &Versor),
    a_i: &Body,
    a_j: &Body,
    b_i: &Body,
    b_j: &Body,
    budget: &mut crate::physics::Budget,
) -> Result<(crate::transducer::sentence::Transport, Carried), Vec<EdgeRefusal>> {
    let union = |x: &Body, y: &Body| -> Shape {
        let compartments: Vec<Compartment> = x.compartments.iter().chain(y.compartments.iter()).cloned().collect();
        blade::shape_of(&sites_of(&compartments))
    };
    // `ṽᵢ vᵢⱼ`, the edge as a ray (78.32(1)).
    crate::transducer::cga::is_versor(v_i).then_some(()).ok_or_else(|| vec![EdgeRefusal::NoReading])?;
    let inv = v_i.reverse();
    // A spent stroke read nothing, and says so. A pair no versor carries
    // moved differentially: its edge is the relative motion, read by
    // `edge_of` as every edge is, its bend counted by kind.
    let members = match blade::versor_coset_ordered(&union(a_i, a_j), &union(b_i, b_j), budget) {
        Ok(members) => members,
        Err(blade::NoPose::Unbounded) => return Err(vec![EdgeRefusal::NoReading]),
        Err(_) => return relative_edge(v_i, v_j),
    };
    let mut refusals: Vec<EdgeRefusal> = Vec::new();
    // The coset in its content order, geodesic first then by each member's
    // own key: the least-keyed member that reads is the canonical one.
    for t in &members {
        match edge_of(&inv.geo(&t.versor)) {
            Ok(transport) => return Ok((transport, Carried::Rigid)),
            Err(e) => refusals.push(e),
        }
    }
    Err(refusals)
}

/// 78.103 — THE RELATIVE MOTION OF TWO BODIES, `Γ = ṽᵢ·vⱼ`, read by
/// `edge_of` as every edge is, its bend counted by kind. It needs the two
/// motions only — a pair with no bodies to measure (a cover's piece) is
/// related by it as a pair no versor carries is. A motion that is no versor
/// is a tear.
pub(crate) fn relative_edge(
    v_i: &Versor,
    v_j: &Versor,
) -> Result<(crate::transducer::sentence::Transport, Carried), Vec<EdgeRefusal>> {
    if !crate::transducer::cga::is_versor(v_i) || !crate::transducer::cga::is_versor(v_j) {
        return Err(vec![EdgeRefusal::NotRigid]);
    }
    let gamma = v_i.reverse().geo(v_j);
    edge_of(&gamma).map(|t| (t, Carried::Relative)).map_err(|e| vec![e])
}

/// 78.103 — HOW AN EDGE'S PAIR WAS CARRIED: as one rigid pair-body, or each
/// body by its own motion, the edge the bodies' relative motion.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Carried {
    Rigid,
    Relative,
}

/// A body's support on the sites of the base, sorted.
pub(crate) fn support_of(body: &Body) -> Vec<Site> {
    sites_of(&body.compartments)
}

// `ring_of_support` STOOD HERE AND IS DELETED — T1 (60.5, ruled
// 2026-09-22). It dilated a support by a Manhattan 4-neighbourhood on ℤ²
// and had exactly two callers: the cover's meet index, purged by 58.3, and
// `star_and_site_faces`, purged with this. Between them they were the
// engine's entire notion of incidence, and both were reading the dilation
// rather than the corpus — 184 of 186 meets, and every `b₁`,
// `orbit_defects` and spectrum taken through the form's faces.
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
    // A negative site is off the grid by `try_from`, never by an index a
    // wrap happened to put out of range.
    painted.iter().all(|&(r, c, h)| {
        let at = |x: i64| usize::try_from(x).ok();
        at(r).and_then(|r| g.get(r))
            .and_then(|row| row.get(at(c)?))
            .copied()
            == Some(h)
    })
}

/// The scene's bodies, each indexed by its own chart (Y4, 24.4, 20).
/// A chart whose fibre holds exactly one body is "the unique one", read as
/// `fibre.iter().nth(1).is_none()` (W10), without a count compared to a literal.
pub fn bodies_by_chart(bodies: &[Body]) -> crate::network::nerve::Keyed<Vec<&Body>> {
    let mut fibres: crate::network::nerve::Keyed<Vec<&Body>> = crate::network::nerve::Keyed::new();
    for b in bodies {
        let k = b.moduli.key();
        match fibres.get_mut(&k) {
            Some(v) => v.push(b),
            None => {
                fibres.insert(k, vec![b]);
            }
        }
    }
    fibres
}

/// The chart whose fibre in the scene holds exactly one body (Y4, 24.4, 21 step 1, W10).
/// Answers "the chart whose fibre holds one" without a count compared to a literal.
pub fn chart_holding_one(bodies: &[Body]) -> Option<crate::transducer::stalk::Invariant> {
    let fibres = bodies_by_chart(bodies);
    let mut singletons = fibres
        .scoped()
        .filter_map(|(_, fibre)| {
            let mut it = fibre.iter();
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
