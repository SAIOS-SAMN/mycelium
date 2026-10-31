// 78.88 — INSTRUMENTATION ONLY (§8.1): does reading a scene by its
// SPOROPHORE — the enclosure its bodies' centres join (`cga::enclosure`), the
// lexicon's "an enclosure, the join of centres, nested" — matter for
// perception now? Per train pair: whether the frame's extent changes (and
// in proportion); the input bodies some output body on the same chart is
// carried onto (`carriage`) and those none is, by whether the extent
// changed; and of the carriages, those whose motion also carries the input
// scene's sporophore onto the output's (the body moved WITH its scene).
// Read projectively, no root. Data; nothing is decided.

use mycelium::transducer::cga::{enclosure, projective_ratio};
use mycelium::transducer::family::{carriage, perceive};

const STROKE: usize = 1 << 20;

#[derive(Default)]
struct Tally {
    pairs: u64,
    resized: u64,
    proportional: u64,
    /// Input bodies carried / uncarried, in pairs whose extent is kept and
    /// in pairs whose extent changed.
    carried_kept: u64,
    uncarried_kept: u64,
    carried_resized: u64,
    uncarried_resized: u64,
    /// Pairs whose sporophores both read (no flat enclosure), and of them
    /// those whose sporophore stands still (one sphere, projectively).
    enclosed: u64,
    still: u64,
    /// Carriages read against both sporophores, and those whose motion
    /// carries the input's onto the output's: in kept and in resized pairs.
    asked_kept: u64,
    with_scene_kept: u64,
    asked_resized: u64,
    with_scene_resized: u64,
    /// The changed extents by kind: the output inside the input on every
    /// axis (cut), around it (grown), a whole multiple of it on every axis
    /// (tiled, a kind of grown), or larger on one axis and smaller on another.
    cut: u64,
    grown: u64,
    tiled: u64,
    mixed: u64,
    /// THE CUT AS A CHAMBER: of the cut pairs, those whose output is an exact
    /// section of the input at some place (and those exact only once hues
    /// are mapped, one function), and of the exact ones, the region one input
    /// body SURROUNDS (a chamber: every site on the ring just outside it that
    /// is in the frame is that body's), the region one whole input body
    /// SPANS (the cut selects that body), or neither.
    cut_exact: u64,
    cut_recoloured: u64,
    cut_none: u64,
    chamber: u64,
    spans_body: u64,
    neither: u64,
    /// THE CUT AS CONTENT CARRIED: of the cut pairs that are no section, the
    /// output's bodies, and those some input body on the same chart is carried
    /// onto; the pairs whose output is ONE body carried from the input, and
    /// those whose every output body is carried.
    cut_out_bodies: u64,
    cut_out_carried: u64,
    one_carried: u64,
    all_carried: u64,
    /// THE RING LAYER, built upward: each input body by the rings its
    /// filaments close — [in a ring with another body, in a ring with the
    /// scene only, carried and open, not carried] — in cut pairs and in all
    /// pairs.
    rings_cut: [u64; 4],
    rings_all: [u64; 4],
    /// THE RING ACROSS THE FAMILY: each input body by whether a filament of
    /// it closes a square with a filament of another pair — [closes across
    /// the family, carried and open across it, not carried, its family
    /// holds no other pair] — in cut pairs and in all; and the carriages a
    /// spent stroke left unread.
    family_cut: [u64; 4],
    family_all: [u64; 4],
    family_unread: u64,
}

type Body = mycelium::transducer::family::Body;
type Filament = (usize, usize, mycelium::transducer::cga::Mv);

/// THE RING ACROSS THE FAMILY, per pair, per input body: the place it
/// stands — 0 a filament of it closes a square with another pair's
/// (`W·V ∝ V′·U`: the body and its image related across the pairs by one
/// transport), 1 carried and open across the family, 2 not carried, 3 the
/// family holds no other pair. Carriages are read once each and kept; a
/// carriage the stroke could not read is counted unread, never "none".
fn family_rings(pairs: &[(Vec<Body>, Vec<Body>)], budget: &mut mycelium::physics::Budget, unread: &mut u64) -> Vec<Vec<usize>> {
    use mycelium::network::nerve::Keyed;
    let mut memo: Keyed<Option<mycelium::transducer::cga::Mv>> = Keyed::new();
    let mut carry = |key: Vec<u8>, a: &Body, b: &Body, budget: &mut mycelium::physics::Budget, unread: &mut u64| -> Option<mycelium::transducer::cga::Mv> {
        if let Some(held) = memo.get(&key) {
            return held.clone();
        }
        if a.moduli.key() != b.moduli.key() {
            memo.insert(&key, None);
            return None;
        }
        if budget.remaining() == 0 {
            *unread += 1;
            return None;
        }
        let v = carriage(a, b, budget).map(|c| c.v);
        memo.insert(&key, v.clone());
        v
    };
    let k = |tag: u8, p: usize, i: usize, q: usize, j: usize| -> Vec<u8> {
        let mut out = vec![tag];
        for x in [p, i, q, j] {
            out.extend_from_slice(&(x as u64).to_be_bytes());
        }
        out
    };
    // Each pair's filaments.
    let filaments: Vec<Vec<Filament>> = pairs
        .iter()
        .enumerate()
        .map(|(p, (ins, outs))| {
            let mut f = Vec::new();
            for (i, a) in ins.iter().enumerate() {
                for (j, b) in outs.iter().enumerate() {
                    if let Some(v) = carry(k(0, p, i, p, j), a, b, budget, unread) {
                        f.push((i, j, v));
                    }
                }
            }
            f
        })
        .collect();
    pairs
        .iter()
        .enumerate()
        .map(|(p, (ins, _))| {
            (0..ins.len())
                .map(|i| {
                    let of_i: Vec<&Filament> = filaments[p].iter().filter(|f| f.0 == i).collect();
                    if of_i.is_empty() {
                        return 2;
                    }
                    if pairs.len() < 2 {
                        return 3;
                    }
                    for (a, b, v) in &of_i {
                        for (q, (ins_q, outs_q)) in pairs.iter().enumerate() {
                            if q == p {
                                continue;
                            }
                            for (a2, b2, v2) in &filaments[q] {
                                let Some(u) = carry(k(1, p, *a, q, *a2), &pairs[p].0[*a], &ins_q[*a2], budget, unread) else { continue };
                                let Some(w) = carry(k(2, p, *b, q, *b2), &pairs[p].1[*b], &outs_q[*b2], budget, unread) else { continue };
                                if projective_ratio(&w.geo(v), &v2.geo(&u)).is_some() {
                                    return 0;
                                }
                            }
                        }
                    }
                    1
                })
                .collect()
        })
        .collect()
}

/// One input body's place on the ring layer of its pair: its filaments
/// (carriages onto output bodies on its chart) by canonical ray, read
/// against every other body's and against the scene's sporophores.
fn ring_layer(
    ins: &[mycelium::transducer::family::Body],
    outs: &[mycelium::transducer::family::Body],
    spores: (&Option<mycelium::transducer::cga::Mv>, &Option<mycelium::transducer::cga::Mv>),
    budget: &mut mycelium::physics::Budget,
) -> Vec<usize> {
    // Every filament: (input, output, its motion's canonical ray, the motion).
    let mut filaments: Vec<(usize, usize, Vec<u8>, mycelium::transducer::cga::Mv)> = Vec::new();
    for (i, a) in ins.iter().enumerate() {
        for (j, b) in outs.iter().enumerate() {
            if a.moduli.key() != b.moduli.key() {
                continue;
            }
            let Some(c) = carriage(a, b, budget) else { continue };
            let Some(ray) = c.v.canonical() else { continue };
            let mut k = Vec::new();
            ray.put_canonical(&mut k);
            filaments.push((i, j, k, c.v));
        }
    }
    (0..ins.len())
        .map(|i| {
            let of_i: Vec<&(usize, usize, Vec<u8>, mycelium::transducer::cga::Mv)> = filaments.iter().filter(|f| f.0 == i).collect();
            if of_i.is_empty() {
                return 3;
            }
            let with_body = of_i.iter().any(|f| filaments.iter().any(|g| g.0 != i && g.1 != f.1 && g.2 == f.2));
            if with_body {
                return 0;
            }
            let with_scene = match spores {
                (Some(si), Some(so)) => of_i.iter().any(|f| projective_ratio(&f.3.sandwich(si), so).is_some()),
                _ => false,
            };
            match with_scene {
                true => 1,
                false => 2,
            }
        })
        .collect()
}

/// The places the output stands in the input as a section: exact, or exact
/// once one hue function is applied (the void kept). Returns (exact offsets,
/// recoloured offsets).
type Places = Vec<(usize, usize)>;

fn sections(x: &mycelium::arc::Grid, y: &mycelium::arc::Grid) -> (Places, Places) {
    let ([hx, wx], [hy, wy]) = (mycelium::arc::extents(x), mycelium::arc::extents(y));
    let (mut exact, mut recoloured) = (Vec::new(), Vec::new());
    if hy > hx || wy > wx || hy == 0 || wy == 0 {
        return (exact, recoloured);
    }
    for r in 0..=hx - hy {
        for c in 0..=wx - wy {
            let mut same = true;
            let mut map: Vec<(u8, u8)> = Vec::new();
            let mut function = true;
            for i in 0..hy {
                for j in 0..wy {
                    let (a, b) = (x[r + i][c + j], y[i][j]);
                    same &= a == b;
                    match map.iter().find(|(f, _)| *f == a) {
                        Some((_, t)) if *t != b => function = false,
                        Some(_) => {}
                        None => map.push((a, b)),
                    }
                }
            }
            // The void stays the void: a recolour does not paint emptiness.
            let void_kept = map.iter().all(|&(f, t)| (f == 0) == (t == 0));
            match (same, function && void_kept) {
                (true, _) => exact.push((r, c)),
                (false, true) => recoloured.push((r, c)),
                _ => {}
            }
        }
    }
    (exact, recoloured)
}

/// What a section at `(r, c)` of extents `(h, w)` is to the input's bodies.
enum Region {
    Chamber,
    SpansBody,
    Neither,
}

fn region(x: &mycelium::arc::Grid, at: (usize, usize), ext: (usize, usize)) -> Region {
    let bodies = perceive(x);
    let sites = |b: &mycelium::transducer::family::Body| -> Vec<(i64, i64)> {
        b.compartments.iter().filter_map(|l| Some((l.site.axis(0)?, l.site.axis(1)?))).collect()
    };
    let (r0, c0, h, w) = (at.0 as i64, at.1 as i64, ext.0 as i64, ext.1 as i64);
    let inside = |&(r, c): &(i64, i64)| r >= r0 && r < r0 + h && c >= c0 && c < c0 + w;
    // The ring just outside the region, those of its sites in the frame.
    let [hx, wx] = mycelium::arc::extents(x);
    let ring: Vec<(i64, i64)> = (r0 - 1..=r0 + h)
        .flat_map(|r| (c0 - 1..=c0 + w).map(move |c| (r, c)))
        .filter(|s| !inside(s))
        .filter(|&(r, c)| usize::try_from(r).is_ok_and(|u| u < hx) && usize::try_from(c).is_ok_and(|u| u < wx))
        .collect();
    let surrounds = !ring.is_empty() && bodies.iter().any(|b| {
        let s = sites(b);
        ring.iter().all(|p| s.contains(p))
    });
    if surrounds {
        return Region::Chamber;
    }
    let spans = bodies.iter().any(|b| {
        let s = sites(b);
        !s.is_empty()
            && s.iter().all(inside)
            && s.iter().any(|p| p.0 == r0)
            && s.iter().any(|p| p.0 == r0 + h - 1)
            && s.iter().any(|p| p.1 == c0)
            && s.iter().any(|p| p.1 == c0 + w - 1)
    });
    match spans {
        true => Region::SpansBody,
        false => Region::Neither,
    }
}

pub(crate) fn report(dir: &str, names: &[String]) {
    let mut t = Tally::default();
    for name in names {
        let Ok(json) = std::fs::read_to_string(format!("{dir}/{name}")) else { continue };
        let Some(task) = mycelium::arc::parse_task(&json) else { continue };
        let mut physics = mycelium::physics::Physics::default();
        // THE RING ACROSS THE FAMILY: the pairs read as one object, every
        // pair's filaments squared against every other pair's.
        let perceived: Vec<(Vec<Body>, Vec<Body>)> = task.train.iter().map(|(x, y)| (perceive(x), perceive(y))).collect();
        let mut family_budget = physics.stroke_n(STROKE);
        let places = family_rings(&perceived, &mut family_budget, &mut t.family_unread);
        for ((x, y), per) in task.train.iter().zip(&places) {
            let (ex, ey) = (mycelium::arc::extents(x), mycelium::arc::extents(y));
            let cut = ex != ey && ey.iter().zip(&ex).all(|(o, i)| o <= i);
            for &place in per {
                t.family_all[place] += 1;
                if cut {
                    t.family_cut[place] += 1;
                }
            }
        }
        for (x, y) in &task.train {
            let mut budget = physics.stroke_n(STROKE);
            t.pairs += 1;
            let (ex, ey) = (mycelium::arc::extents(x), mycelium::arc::extents(y));
            let resized = ex != ey;
            t.resized += u64::from(resized);
            // In proportion: one ratio on every axis.
            t.proportional += u64::from(resized && ex[0] * ey[1] == ex[1] * ey[0]);
            if resized {
                let (le, ge) = (ey.iter().zip(&ex).all(|(o, i)| o <= i), ey.iter().zip(&ex).all(|(o, i)| o >= i));
                match (le, ge) {
                    (true, _) => {
                        t.cut += 1;
                        let (exact, recoloured) = sections(x, y);
                        match (exact.as_slice(), recoloured.is_empty()) {
                            ([], true) => {
                                t.cut_none += 1;
                                let (ins, outs) = (perceive(x), perceive(y));
                                let carried: Vec<bool> = outs
                                    .iter()
                                    .map(|b| ins.iter().any(|a| a.moduli.key() == b.moduli.key() && carriage(a, b, &mut budget).is_some()))
                                    .collect();
                                t.cut_out_bodies += outs.len() as u64;
                                t.cut_out_carried += carried.iter().filter(|c| **c).count() as u64;
                                t.one_carried += u64::from(matches!(carried.as_slice(), [true]));
                                t.all_carried += u64::from(!carried.is_empty() && carried.iter().all(|c| *c));
                            }
                            ([], false) => t.cut_recoloured += 1,
                            (places, _) => {
                                t.cut_exact += 1;
                                // Every place the section stands is read; the
                                // pair counts by the strongest reading any
                                // place gives, none chosen over another.
                                let reads: Vec<Region> = places.iter().map(|&p| region(x, p, (ey[0], ey[1]))).collect();
                                match () {
                                    _ if reads.iter().any(|r| matches!(r, Region::Chamber)) => t.chamber += 1,
                                    _ if reads.iter().any(|r| matches!(r, Region::SpansBody)) => t.spans_body += 1,
                                    _ => t.neither += 1,
                                }
                            }
                        }
                    }
                    (_, true) => {
                        t.grown += 1;
                        t.tiled += u64::from(ey.iter().zip(&ex).all(|(o, i)| *i > 0 && o % i == 0));
                    }
                    _ => t.mixed += 1,
                }
            }
            let (ins, outs) = (perceive(x), perceive(y));
            let nuclei = |bs: &[mycelium::transducer::family::Body]| -> Vec<_> { bs.iter().filter_map(|b| b.shape.blade.vertex_nucleus()).collect() };
            let spores = (enclosure(&nuclei(&ins)), enclosure(&nuclei(&outs)));
            if let (Some(si), Some(so)) = &spores {
                t.enclosed += 1;
                t.still += u64::from(projective_ratio(si, so).is_some());
            }
            // THE RING LAYER, built upward from the filaments.
            let cut = resized && ey.iter().zip(&ex).all(|(o, i)| o <= i);
            for place in ring_layer(&ins, &outs, (&spores.0, &spores.1), &mut budget) {
                t.rings_all[place] += 1;
                if cut {
                    t.rings_cut[place] += 1;
                }
            }
            for a in &ins {
                let mut any = false;
                for b in &outs {
                    if a.moduli.key() != b.moduli.key() {
                        continue;
                    }
                    let Some(c) = carriage(a, b, &mut budget) else { continue };
                    any = true;
                    if let (Some(si), Some(so)) = &spores {
                        let with = projective_ratio(&c.v.sandwich(si), so).is_some();
                        match resized {
                            true => {
                                t.asked_resized += 1;
                                t.with_scene_resized += u64::from(with);
                            }
                            false => {
                                t.asked_kept += 1;
                                t.with_scene_kept += u64::from(with);
                            }
                        }
                    }
                }
                match (resized, any) {
                    (false, true) => t.carried_kept += 1,
                    (false, false) => t.uncarried_kept += 1,
                    (true, true) => t.carried_resized += 1,
                    (true, false) => t.uncarried_resized += 1,
                }
            }
        }
    }
    println!(
        "sporophore: pairs {} | extent changed {} (in proportion {}) | sporophores read on both sides {} (standing still {})",
        t.pairs, t.resized, t.proportional, t.enclosed, t.still
    );
    println!("sporophore extent changes: cut {} | grown {} (whole multiples, tiled {}) | mixed {}", t.cut, t.grown, t.tiled, t.mixed);
    println!(
        "sporophore cuts: an exact section of the input {} (a chamber one body surrounds {} | one whole body spanned {} | neither {}) | a section once hues are mapped {} | no section {}",
        t.cut_exact, t.chamber, t.spans_body, t.neither, t.cut_recoloured, t.cut_none
    );
    println!(
        "sporophore cuts that are no section: output bodies {} carried from an input body {} | outputs that are ONE carried body {} | outputs whose every body is carried {}",
        t.cut_out_bodies, t.cut_out_carried, t.one_carried, t.all_carried
    );
    for (what, r) in [("cut pairs", t.family_cut), ("all pairs", t.family_all)] {
        println!(
            "sporophore ring across the family, {what}: input bodies closing a square with another pair {} | carried, open across the family {} | not carried {} | family of one pair {}",
            r[0], r[1], r[2], r[3]
        );
    }
    println!("sporophore ring across the family: carriages a spent stroke left unread {}", t.family_unread);
    for (what, r) in [("cut pairs", t.rings_cut), ("all pairs", t.rings_all)] {
        println!(
            "sporophore ring layer, {what}: input bodies in a ring with another body {} | in a ring with the scene only {} | carried, open {} | not carried {}",
            r[0], r[1], r[2], r[3]
        );
    }
    println!(
        "sporophore bodies: extent kept — carried {} uncarried {} | extent changed — carried {} uncarried {}",
        t.carried_kept, t.uncarried_kept, t.carried_resized, t.uncarried_resized
    );
    println!(
        "sporophore carriages moving WITH the scene: extent kept {} of {} | extent changed {} of {}",
        t.with_scene_kept, t.asked_kept, t.with_scene_resized, t.asked_resized
    );
}
