// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// MU-SENSING §3.4, §8; ROADMAP track 26 S2 — THE GROUND CENSUS: over the
// training split, the deduced ground (`raster::ground_of`) beside the
// count the former `ground_of` took (0 where any site shows 0, else the
// symbol of the greatest site count, else 0 on a tie); the tasks the
// former decoder's frame fixing turned; and the task addresses that moved
// from the former identity (frame-fixed train frames' bands, test inputs'
// chains off 0) to the samples' content as posted. A census, read with
// `--nocapture`; it decides nothing.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use mycelium::transducer::address::{put_key_bytes, Address, TAG_TASK};
use mycelium::transducer::family::transduce::{Candidacy, Enclosed, Ground};
use mycelium::transducer::family::{Family, Task};
use mycelium::transducer::raster::{frame_fixed, ground_of, on_ground};
use mycelium_source_arc_agi::{corpus, decode, task_names};

/// The former count, kept here alone to read what S2 changed.
fn former_ground(family: &Family) -> u8 {
    let mut count = [0u64; 256];
    for (x, y) in family {
        for &h in x.symbols().iter().chain(y.symbols()) {
            count[usize::from(h)] += 1;
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

/// The former identity, kept here alone to count what S2 moved: each train
/// pair's frames as their bands, each test input's chain off 0, over the
/// task the former decoder posted (its frame fixed on the former ground).
fn former_address(task: &Task) -> Option<Address> {
    let mut buf = vec![TAG_TASK];
    for (a, b) in &task.train {
        let mut pair = Vec::new();
        for g in [a, b] {
            Enclosed::of(g.sample()).put_content(&mut pair);
        }
        put_key_bytes(&mut buf, &pair);
    }
    for (x, _) in &task.test {
        let chain: Vec<mycelium::transducer::sentence::SectionLink> = x
            .sites()
            .filter(|&(_, h)| h != 0)
            .map(|(site, h)| mycelium::transducer::sentence::SectionLink::new(site, h, h))
            .collect();
        put_key_bytes(&mut buf, &mycelium::transducer::element::from_residue(&chain)?.address());
    }
    Some(Address::new(buf))
}

#[test]
fn ground_census_over_training() {
    let dir = corpus("training");
    let names = match task_names(&dir) {
        Ok(names) => names,
        Err(e) => panic!("{dir} does not read ({e}): no corpus, no census; set MYCELIUM_CORPUS to the ARC-AGI-2 data directory or place it at data/arc-agi-2"),
    };
    let (mut same, mut other, mut vortex, mut none) = (Vec::new(), Vec::new(), Vec::new(), Vec::new());
    let (mut turned, mut moved, mut unaddressed) = (0usize, 0usize, 0usize);
    for name in names {
        let Ok(json) = std::fs::read_to_string(format!("{dir}/{name}")) else { continue };
        let Some(task) = decode(&json) else { continue };
        let old = former_ground(&task.train);
        let new = ground_of(&task.train);
        let former = on_ground(Ground::One(old), || frame_fixed(task.clone()));
        if former != task {
            turned += 1;
        }
        match (former_address(&former), mycelium::registry::task_address(&task)) {
            (Some(a), Some(b)) if a == b => {}
            (Some(_), Some(_)) => moved += 1,
            _ => unaddressed += 1,
        }
        match &new {
            Ground::One(g) if *g == old => same.push(name),
            Ground::One(g) => other.push(format!("{name} (was {old}, reads {g})")),
            Ground::Vortex(gs) => vortex.push(format!("{name} (was {old}, reads {gs:?})")),
            Ground::None => none.push(format!("{name} (was {old})")),
        }
    }
    let total = same.len() + other.len() + vortex.len() + none.len();
    // THE CONDITION EACH NONE TASK'S FORMER GROUND FAILS (MU-SENSING §3.4),
    // and every symbol's candidacy on five of them.
    let (mut absent, mut unenclosing, mut carried) = (Vec::new(), Vec::new(), Vec::new());
    for line in &none {
        let Some(name) = line.split(' ').next().map(str::to_string) else { continue };
        let Ok(json) = std::fs::read_to_string(format!("{dir}/{name}")) else { continue };
        let Some(task) = decode(&json) else { continue };
        let pairs: Vec<_> = task.train.iter().map(|(x, y)| (x.sample().clone(), y.sample().clone())).collect();
        let former = former_ground(&task.train);
        let readings = Ground::candidacies(&pairs);
        if ["017c7c7b.json", "045e512c.json", "05269061.json", "0a1d4ef5.json", "0c786b71.json"].contains(&name.as_str()) {
            println!("candidacies {name}: {readings:?}");
        }
        match readings.iter().find(|(s, _)| s.as_slice() == [former]).map(|(_, c)| *c) {
            Some(Candidacy::Absent { .. }) => absent.push(name),
            Some(Candidacy::Unenclosing { .. }) => unenclosing.push(name),
            Some(Candidacy::CarriedAsBody { .. }) => carried.push(name),
            _ => {}
        }
    }
    println!(
        "None tasks, former ground's failed condition: absent {}, unenclosing {} {:?}, carried as a body {}",
        absent.len(),
        unenclosing.len(),
        unenclosing,
        carried.len()
    );
    println!(
        "tasks {total}: One same {}, One other {}, Vortex {}, None {}; former frame fixing turned {turned}; addresses moved {moved}, unaddressed on either side {unaddressed}",
        same.len(),
        other.len(),
        vortex.len(),
        none.len()
    );
    for (label, list) in [("One other", &other), ("Vortex", &vortex), ("None", &none)] {
        println!("{label}:");
        for line in list.iter() {
            println!("  {line}");
        }
    }
}
