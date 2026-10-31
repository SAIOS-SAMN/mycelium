// Z10's calibration — THE WALK'S FIGURE AGAINST THE ALLOCATOR'S, on one
// object counted both ways.
//
//! The completed vector reaches 50% of the allocator. Three things could
//! account for the other half and they call for different fixes: the `ℚ`
//! approximation understating every rational, a carrier still unlisted, or
//! the allocator's own overhead. One measurement separates them.
//!
//! A VERSOR isolates the arithmetic: it is thirty-two rationals and
//! nothing else, so if the walk is light here the residue is `bytes_q` and
//! the vector's arithmetic is corrected once. A FORM adds every carrier a
//! form owns, so if the walk is light there but not on the versor, a
//! carrier is missing and it can be named. If both agree, the residue is
//! the allocator's and is a fact rather than something to chase.
//!
//! THIS IS NOT A MESH READING and does not claim to be. It measures the
//! arithmetic of `Bytes`, which is a property of the code; the mesh's own
//! numbers come from a recut and nowhere else.

use mycelium::transducer::family::{extract_rule_on, extraction_spend, perceive_family};
use mycelium::transducer::bytes::Bytes;

fn live() -> i128 {
    match tikv_jemalloc_ctl::epoch::advance() {
        Ok(_) => match tikv_jemalloc_ctl::stats::allocated::read() {
            Ok(n) => n as i128,
            Err(_) => 0,
        },
        Err(_) => 0,
    }
}

/// The walk against the allocator, per ten thousand, exactly. A zero
/// denominator is an unread allocator and reports nothing rather than a
/// number (rule 11).
fn ratio(walk: i128, alloc: i128) -> String {
    match alloc {
        0 => "unread".to_string(),
        d => (walk * 10_000 / d).to_string(),
    }
}

/// The corpus file whose registry task id begins with `want` — the map the
/// mesh's journal needs, since a voice names a family by its registry id
/// and the census must read that family's own file.
pub(crate) fn find_id(dir: &str, names: &[String], want: &str) {
    for file in names {
        let Ok(json) = std::fs::read_to_string(format!("{dir}/{file}")) else { continue };
        let Some(task) = mycelium::arc::parse_task(&json) else { continue };
        let Some(id) = mycelium::server::registry::task_id(&task) else { continue };
        let hx: String = id.as_bytes().iter().take(2).map(|b| format!("{b:02x}")).collect();
        if hx != want {
            continue;
        }
        // And its form, which is what the census is for: the reading is
        // the family's own size, not the id.
        let epi = mycelium::client::sdk::Epigenome::new();
        let mut physics = mycelium::physics::Physics::default();
        let frames = perceive_family(&task.train);
        let mut b = physics.stroke_n(extraction_spend(&task.train, &frames));
        match extract_rule_on(&task.train, &frames, &epi, &mut b) {
            Some(r) => println!(
                "  id {hx} = {file}: {} vertices {} faces {} orbits",
                r.form.vertices.len(),
                r.form.faces.len(),
                r.form.orbits.len()
            ),
            None => println!("  id {hx} = {file}: withheld"),
        }
    }
}

pub(crate) fn report(dir: &str, name: &str) {
    // ── ONE VERSOR, a thousand times. A single one is smaller than the
    // allocator's granularity; a thousand held at once is not, and the
    // per-object figure is the difference divided by the count — which is
    // arithmetic on a measurement, not a measurement of one.
    const N: usize = 1000;
    let a0 = live();
    let mut held: Vec<mycelium::transducer::element::Versor> = Vec::with_capacity(N);
    for i in 0..N {
        held.push(mycelium::transducer::element::translator_versor(i as i64, (i * 7) as i64));
    }
    let a1 = live();
    let walked: usize = held.iter().map(Bytes::bytes).sum();
    let allocated = a1 - a0;
    // THE RATIO IS EXACT (law 4). A float here would be a rounded reading
    // of a measurement, in a file whose whole job is to say how close two
    // measurements are; per ten thousand, in integers, says it.
    println!(
        "  calibrate versor x{N}: walk {walked} B  allocator {allocated} B  walk/alloc {}/10000",
        ratio(walked as i128, allocated)
    );
    drop(held);

    // ── ONE FORM. The allocator's delta around a clone against the walk's
    // figure for the same object.
    let Ok(json) = std::fs::read_to_string(format!("{dir}/{name}")) else {
        eprintln!("  calibrate refused: {dir}/{name} does not read");
        return;
    };
    let Some(task) = mycelium::arc::parse_task(&json) else {
        eprintln!("  calibrate refused: not a task");
        return;
    };
    let epi = mycelium::client::sdk::Epigenome::new();
    let mut physics = mycelium::physics::Physics::default();
    let frames = perceive_family(&task.train);
    let mut b = physics.stroke_n(extraction_spend(&task.train, &frames));
    let Some(rule) = extract_rule_on(&task.train, &frames, &epi, &mut b) else {
        eprintln!("  calibrate refused: {name} extracts no rule");
        return;
    };
    let f0 = live();
    let copy = rule.form.clone();
    let f1 = live();
    let walked = copy.bytes();
    let allocated = f1 - f0;
    println!(
        "  calibrate form {name}: {} vertices {} faces  walk {walked} B  allocator {allocated} B  walk/alloc {}/10000",
        copy.vertices.len(),
        copy.faces.len(),
        ratio(walked as i128, allocated)
    );
}
