// mycelium#87, D2.5 — THE CHART'S INTAKE COST ON ONE NAMED TASK: what
// `invariant_of` and the intake's pieces cost, pair by pair, and the bit
// lengths of the exact coordinates the chart carries.
//
//! The intake reads each train pair once (`route::routes_of`): the pair's
//! residue, its chart (`stalk::invariant_of`, which for a residue word is
//! `blade::section_moduli` on the support) and the pieces the court lists
//! (`cognition::prepare_intake`), each charted. This walks the same calls
//! on every train pair of the named task, whether or not the pair's two
//! enclosures share periods (`routes_of` routes only those that do), and
//! times them apart: the lift of the support, its topology and its
//! harmonics, so the stage that costs is read rather than named.
//!
//! THE CLOCK IS THIS BINARY'S, NEVER A KERNEL'S (as `regime.rs`): no engine
//! branch reads these numbers, and the process prints them and exits.
//!
//! TRACE LINES AND NOTHING ELSE (MU-STANDARDS 8.1). No library function is
//! changed; if this file were deleted the engine would behave identically.

#![allow(
    clippy::disallowed_types,
    reason = "mycelium#87: the chart's per-pair intake cost in seconds; law 9 binds kernels and this is an instrument"
)]

use mycelium::transducer::stalk::ChartAxis;
use std::time::{Duration, Instant};

/// A duration as seconds and milliseconds, the instrument's one format.
fn secs(d: Duration) -> String {
    format!("{}.{:03}s", d.as_secs(), d.subsec_millis())
}

/// Each ratio coordinate of a chart as its numerator's and denominator's bit
/// lengths, in axis order; a count is written as itself.
fn widths(axes: &[ChartAxis]) -> String {
    axes.iter()
        .map(|a| match a {
            ChartAxis::Count(n) => format!("{n}"),
            ChartAxis::Ratio(q) => format!("{}/{}b", q.numer().bits(), q.denom().bits()),
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Read the named task's train pairs through the intake's calls.
pub(crate) fn report(dir: &str, name: &str) {
    let Ok(posted) = std::fs::read(format!("{dir}/{name}")) else {
        eprintln!("  chart refused: {dir}/{name} does not read");
        return;
    };
    let Some(section) = mycelium::anastome::dialogue::decode_posted_task(&posted) else {
        eprintln!("  chart refused: not a task on the alphabet's section");
        return;
    };
    let t0 = Instant::now();
    let routes = mycelium::server::route::routes_of(&section);
    println!("  chart {name}: routes_of {} routes of {} pairs in {}", routes.len(), section.train.len(), secs(t0.elapsed()));
    // Every pair is read on the family's own ground, as the mind and the
    // court's reconstruction read it.
    mycelium::transducer::raster::on_ground_of(&section.train, || pairs(name, &section.train));
}

/// THE PAIR'S CHAIN AS THE ENGINE READS IT: the planar residue where the two
/// enclosures share periods (the intake's route), else the unspoken residue
/// a demand carries (`deflation::unspoken_residue`, rebuilt here from its
/// public parts): the transduction's projection, or the output against
/// nothing where the input's turns do not land.
fn chain_of(
    a: &mycelium::transducer::spelled::SectionSample,
    b: &mycelium::transducer::spelled::SectionSample,
) -> (&'static str, Result<Vec<mycelium::transducer::sentence::SectionLink>, mycelium::transducer::family::transduce::Unground>) {
    use mycelium::transducer::family::speak::residue_of;
    use mycelium::transducer::family::transduce::{need_of, Enclosure, Transduction};
    if Enclosure::of(a.sample()).is_same_periods(&Enclosure::of(b.sample())) {
        return ("planar", residue_of(Some(a), b));
    }
    let need = match need_of(a.sample(), b.sample()) {
        Some(n) => n,
        None => 0,
    };
    let mut budget = mycelium::physics::Physics::default().stroke_n(need);
    match Transduction::of(a.sample(), b.sample(), &mut budget).and_then(|t| t.projected()) {
        Ok(changes) => ("projected", Ok(mycelium::transducer::raster::links_of(&changes))),
        Err(_) => ("unspoken", residue_of(None, b)),
    }
}

/// Each train pair, charted by stage.
fn pairs(name: &str, train: &mycelium::transducer::family::Family) {
    for (pair, (a, b)) in train.iter().enumerate() {
        let ([h, w], [h2, w2]) = (a.extents(), b.extents());
        let t = Instant::now();
        let (reading, chain) = chain_of(a, b);
        let chain = match chain {
            Ok(c) => c,
            Err(e) => {
                println!("  chart {name} pair {pair}: {h}x{w} -> {h2}x{w2} {reading}: residue refused {e:?}");
                continue;
            }
        };
        let t_residue = t.elapsed();
        let frame = mycelium::transducer::element::Frame::new(h, w, h2, w2);
        let t = Instant::now();
        let Some(residue) = mycelium::transducer::element::from_residue_framed(&chain, &frame) else {
            println!("  chart {name} pair {pair}: {} links, no canonical chain", chain.len());
            continue;
        };
        let t_word = t.elapsed();
        // THE STAGES OF `section_moduli`, apart: the lift, the topology, the
        // harmonics (which reads the topology again).
        let mut sites = mycelium::transducer::element::positions(&chain);
        sites.sort_unstable();
        sites.dedup();
        let t = Instant::now();
        let shape = mycelium::transducer::blade::shape_of_section(&sites);
        let t_lift = t.elapsed();
        let t = Instant::now();
        let betti = mycelium::transducer::blade::topology(&shape.blade.support);
        let t_topology = t.elapsed();
        let t = Instant::now();
        let heard = mycelium::transducer::blade::harmonics::Harmonics::of(&shape.blade);
        let t_harmonics = t.elapsed();
        let t = Instant::now();
        let chart = mycelium::transducer::stalk::invariant_of(&residue);
        let t_chart = t.elapsed();
        println!(
            "  chart {name} pair {pair}: {h}x{w} -> {h2}x{w2} {reading}, {} links {} sites; residue {} word {} | lift {} topology {} ({:?}) harmonics {} ({}) | invariant_of {}",
            chain.len(),
            sites.len(),
            secs(t_residue),
            secs(t_word),
            secs(t_lift),
            secs(t_topology),
            betti,
            secs(t_harmonics),
            heard.is_some(),
            secs(t_chart)
        );
        println!("  chart {name} pair {pair}: axes [{}]", widths(&chart.0));
        let inertia: Vec<String> = shape.blade.inertia.iter().map(|q| format!("{}/{}b", q.numer().bits(), q.denom().bits())).collect();
        println!("  chart {name} pair {pair}: inertia [{}]", inertia.join(" "));
        let t = Instant::now();
        let pieces = mycelium::cognition::prepare_intake(&chain, &frame);
        let t_pieces = t.elapsed();
        let widest = pieces
            .iter()
            .flat_map(|(_, c)| c.0.iter())
            .filter_map(|a| match a {
                ChartAxis::Ratio(q) => Some(q.numer().bits().max(q.denom().bits())),
                ChartAxis::Count(_) => None,
            })
            .fold(0u64, u64::max);
        println!(
            "  chart {name} pair {pair}: prepare_intake {} pieces in {}, widest ratio {widest} bits",
            pieces.len(),
            secs(t_pieces)
        );
    }
}
