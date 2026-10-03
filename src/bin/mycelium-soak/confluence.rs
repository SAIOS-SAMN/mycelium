// 78.78 — INSTRUMENTATION ONLY (§8.1): P1, confluence over a corpus slice. Per
// task, the rule's remainder read body by body (`family::confluence`): the
// unsaid output bodies standing on no input (creation), one (a single
// carriage) or several (a confluence), and each confluence's mass, centre,
// inverse generator, reproduction, containment and closure. Data; nothing is
// decided.

use mycelium::client::sdk::Epigenome;
use mycelium::transducer::family::confluence::{read_pair, Closes};
use mycelium::transducer::family::{extract_rule_with, form_deflation};

const STROKE: usize = 1 << 20;

pub(crate) fn report(dir: &str, names: &[String]) {
    let empty = Epigenome::new();
    let (mut unframed, mut creation_sites, mut single_sites, mut confluence_sites) = (0u64, 0u64, 0u64, 0u64);
    let (mut creations, mut singles, mut confluences) = (0u64, 0u64, 0u64);
    let (mut mass_kept, mut centre_kept, mut ratio, mut reproduces, mut contains) = (0u64, 0u64, 0u64, 0u64, 0u64);
    let mut regimes: Vec<(&'static str, u64)> = Vec::new();
    let mut closes = [0u64; 5];
    let mut inputs_hist = [0u64; 4];
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
        let mut shown = 0usize;
        let (mut t_cre, mut t_sin, mut t_con) = (0u64, 0u64, 0u64);
        for ((x, y), rem) in task.train.iter().zip(&rems) {
            let mut b = physics.stroke_n(STROKE);
            let p = read_pair(x, y, rem, &mut b);
            unframed += u64::from(p.unframed);
            creations += p.creations;
            singles += p.singles;
            creation_sites += p.creation_sites;
            single_sites += p.single_sites;
            confluence_sites += p.confluence_sites;
            t_cre += p.creation_sites;
            t_sin += p.single_sites;
            t_con += p.confluence_sites;
            for c in &p.confluences {
                confluences += 1;
                mass_kept += u64::from(c.mass_in == c.mass_out);
                centre_kept += u64::from(c.centre_kept);
                ratio += u64::from(c.ratio.is_some());
                reproduces += u64::from(c.reproduces);
                contains += u64::from(c.contains);
                match regimes.iter_mut().find(|(n, _)| *n == c.regime) {
                    Some((_, k)) => *k += 1,
                    None => regimes.push((c.regime, 1)),
                }
                closes[match c.closes {
                    Closes::Finite => 0,
                    Closes::Affine => 1,
                    Closes::Fluid => 2,
                    Closes::Unbounded => 3,
                    Closes::Unread => 4,
                }] += 1;
                inputs_hist[match c.inputs {
                    2 => 0,
                    3 => 1,
                    4..=8 => 2,
                    _ => 3,
                }] += 1;
                if shown < 3 {
                    shown += 1;
                    println!(
                        "    confluence inputs {} mass {} → {} centre kept {} | generator {} ratio {:?} | reproduces {} contains {} | closes {:?} | unsaid {}",
                        c.inputs, c.mass_in, c.mass_out, c.centre_kept, c.regime, c.ratio, c.reproduces, c.contains, c.closes, c.unsaid
                    );
                }
            }
        }
        println!("  confluence {name}: unsaid by class: creation {t_cre} single {t_sin} confluence {t_con}");
    }
    println!(
        "confluence: unsaid by class: creation {creation_sites} ({creations} bodies) | single carriage {single_sites} ({singles}) | confluence {confluence_sites} ({confluences}) | pairs of another frame {unframed}"
    );
    println!(
        "confluence bodies {confluences}: inputs 2: {} 3: {} 4–8: {} more: {} | mass kept {mass_kept} centre kept {centre_kept} (concentric ratio read {ratio}) | the inverse generator reproduces {reproduces} | inputs contained {contains}",
        inputs_hist[0], inputs_hist[1], inputs_hist[2], inputs_hist[3]
    );
    println!("confluence generators by regime: {regimes:?}");
    println!(
        "confluence closure: finite {} affine (crystal) {} fluid {} unbounded {} unread {}",
        closes[0], closes[1], closes[2], closes[3], closes[4]
    );
}
