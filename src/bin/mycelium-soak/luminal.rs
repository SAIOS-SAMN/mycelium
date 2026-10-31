// 78.77 — INSTRUMENTATION ONLY (§8.1): L1–L6 over a corpus slice. Per task:
// the rule extracted, spoken over its training pairs, and what it left unsaid
// read as light (`family::luminal`) — rays, each with its phase, period and
// waveform, interfaces, captured standing waves, cavity billiards — and the
// remainder scene's extraction read by the cover's own tally (L6) and its
// demands by kind. Printed as data; nothing is decided.

use mycelium::client::sdk::Epigenome;
use mycelium::network::nerve::coord::BivectorKind;
use mycelium::transducer::family::invert::{remainder_scene, Painted};
use mycelium::transducer::family::luminal::{read_pair, Beyond, Stop};
use mycelium::transducer::family::{extract_rule_with, form_deflation};
use mycelium::transducer::sentence::{Demand, Vertex};

const STROKE: usize = 1 << 20;

#[derive(Default)]
struct Totals {
    unsaid: u64,
    on_ray: u64,
    captured: u64,
    both: u64,
    neither: u64,
    unframed_pairs: u64,
    unframed_unsaid: u64,
    rays: u64,
    edge: u64,
    transmitted: u64,
    reflected: u64,
    absorbed: u64,
    period_one: u64,
    period_two: u64,
    period_more: u64,
    source_hue: u64,
    cavities: u64,
    rect: u64,
    filled: u64,
    fits: u64,
    misfit: u64,
    parabolic: u64,
    other_modes: u64,
}

impl Totals {
    fn explained(&self) -> u64 {
        self.on_ray + self.captured + self.both
    }
}

pub(crate) fn report(dir: &str, names: &[String]) {
    let empty = Epigenome::new();
    let mut all = Totals::default();
    // Coverage by the remainder's dominant demand kind: deformation, source,
    // creation, none.
    let mut by_kind: [(u64, u64); 4] = [(0, 0); 4];
    for name in names {
        let Ok(json) = std::fs::read_to_string(format!("{dir}/{name}")) else { continue };
        let Some(task) = mycelium::arc::parse_task(&json) else { continue };
        let mut physics = mycelium::physics::Physics::default();
        let mut budget = physics.stroke_n(STROKE);
        let Some(rule) = extract_rule_with(&task.train, &empty, &mut budget) else { continue };
        let (_, rems) = form_deflation(&rule.form, &task.train, &empty);
        if rems.iter().all(Vec::is_empty) {
            continue;
        }
        let mut t = Totals::default();
        let mut lines: Vec<String> = Vec::new();
        for ((x, y), rem) in task.train.iter().zip(&rems) {
            let mut b = physics.stroke_n(STROKE);
            let p = read_pair(x, y, rem, &mut b);
            t.unsaid += p.unsaid;
            t.on_ray += p.on_ray;
            t.captured += p.captured;
            t.both += p.both;
            t.neither += p.neither;
            t.unframed_pairs += u64::from(p.unframed);
            if p.unframed {
                t.unframed_unsaid += p.unsaid;
            }
            for r in &p.rays {
                t.rays += 1;
                match &r.stop {
                    Stop::Edge => t.edge += 1,
                    Stop::Body { beyond: Beyond::Transmitted { .. }, .. } => t.transmitted += 1,
                    Stop::Body { beyond: Beyond::Reflected { .. }, .. } => t.reflected += 1,
                    Stop::Body { beyond: Beyond::Absorbed, .. } => t.absorbed += 1,
                }
                match r.period {
                    1 => t.period_one += 1,
                    2 => t.period_two += 1,
                    _ => t.period_more += 1,
                }
                // The waveform's first hue against its source's: the light is the
                // body's own colour, read by the slice pattern, never by position.
                t.source_hue += u64::from(matches!(r.wave.as_slice(), [h, ..] if *h == r.source));
            }
            for c in &p.cavities {
                t.cavities += 1;
                t.rect += u64::from(c.rect.is_some());
                t.filled += u64::from(c.periods.is_some());
                match c.fits {
                    Some(true) => t.fits += 1,
                    Some(false) => t.misfit += 1,
                    None => {}
                }
                if let Some(ms) = &c.modes {
                    if ms.iter().all(|k| *k == BivectorKind::Parabolic) {
                        t.parabolic += 1;
                    } else {
                        t.other_modes += 1;
                    }
                }
                if c.explains > 0 {
                    lines.push(format!(
                        "    cavity wall {:?} sites {} rect {:?} standing {:?} fits {:?} billiard {:?} captures {}",
                        c.wall, c.sites, c.rect, c.periods, c.fits, c.modes, c.explains
                    ));
                }
            }
            let mut best = p.rays.clone();
            #[expect(clippy::disallowed_methods, reason = "rule 4 — an order by the rays' own content: how many unsaid sites each explains, the instrument's print order")]
            best.sort_by_key(|r| std::cmp::Reverse(r.explains));
            for r in best.iter().take(2) {
                lines.push(format!(
                    "    ray from hue {} dir {:?} reach {} lit {} period {} wave {:?} stop {:?} explains {}",
                    r.source, r.dir, r.reach, r.lit, r.period, r.wave, r.stop, r.explains
                ));
            }
        }
        // L6 — the remainder scene, extracted, and its demands by kind.
        let (scene, _) = remainder_scene(&task.train, &rems, Painted::Sites);
        let mut b = physics.stroke_n(STROKE);
        let kinds = match extract_rule_with(&scene, &empty, &mut b) {
            Some(r) => r.form.vertices.entries().fold([0u64; 3], |mut k, (_, v)| {
                if let Vertex::Demand { kind, .. } = v {
                    match kind {
                        Demand::TopologicalDeformation { .. } => k[0] += 1,
                        Demand::Source { .. } => k[1] += 1,
                        Demand::Creation => k[2] += 1,
                        _ => {}
                    }
                }
                k
            }),
            None => [0; 3],
        };
        let dominant = match kinds {
            [0, 0, 0] => 3,
            [d, s, c] if d >= s && d >= c => 0,
            [_, s, c] if s >= c => 1,
            _ => 2,
        };
        by_kind[dominant].0 += t.unsaid;
        by_kind[dominant].1 += t.explained();
        println!(
            "  light {name}: unsaid {} | on a ray {} captured {} both {} neither {} | rays {} (edge {} transmitted {} reflected {} absorbed {}) | cavities {} | remainder demands deformation {} source {} creation {}",
            t.unsaid, t.on_ray, t.captured, t.both, t.neither, t.rays, t.edge, t.transmitted, t.reflected, t.absorbed, t.cavities, kinds[0], kinds[1], kinds[2]
        );
        for l in lines.iter().take(6) {
            println!("{l}");
        }
        for (a, v) in [
            (&mut all.unsaid, t.unsaid),
            (&mut all.on_ray, t.on_ray),
            (&mut all.captured, t.captured),
            (&mut all.both, t.both),
            (&mut all.neither, t.neither),
            (&mut all.unframed_pairs, t.unframed_pairs),
            (&mut all.unframed_unsaid, t.unframed_unsaid),
            (&mut all.rays, t.rays),
            (&mut all.edge, t.edge),
            (&mut all.transmitted, t.transmitted),
            (&mut all.reflected, t.reflected),
            (&mut all.absorbed, t.absorbed),
            (&mut all.period_one, t.period_one),
            (&mut all.period_two, t.period_two),
            (&mut all.period_more, t.period_more),
            (&mut all.source_hue, t.source_hue),
            (&mut all.cavities, t.cavities),
            (&mut all.rect, t.rect),
            (&mut all.filled, t.filled),
            (&mut all.fits, t.fits),
            (&mut all.misfit, t.misfit),
            (&mut all.parabolic, t.parabolic),
            (&mut all.other_modes, t.other_modes),
        ] {
            *a += v;
        }
    }
    let a = &all;
    println!(
        "light: unsaid {} | explained {} = on a ray {} + captured {} + both {} | neither {} (of them in pairs of another frame, where no ray is read: {} over {} pairs)",
        a.unsaid,
        a.explained(),
        a.on_ray,
        a.captured,
        a.both,
        a.neither,
        a.unframed_unsaid,
        a.unframed_pairs
    );
    println!(
        "light rays: {} | stop edge {} transmitted {} reflected {} absorbed {} | period 1 {} 2 {} more {} | the source's own hue {}",
        a.rays, a.edge, a.transmitted, a.reflected, a.absorbed, a.period_one, a.period_two, a.period_more, a.source_hue
    );
    println!(
        "light cavities: {} | rectangular {} | standing wave {} (fits {} misfit {}) | billiard parabolic {} other {}",
        a.cavities, a.rect, a.filled, a.fits, a.misfit, a.parabolic, a.other_modes
    );
    let label = ["deformation", "source", "creation", "no demand"];
    for (i, (u, e)) in by_kind.iter().enumerate() {
        println!("light by dominant remainder demand {}: unsaid {u} explained {e}", label[i]);
    }
}
