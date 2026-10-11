// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! MU-STANDARDS 8.1 — INSTRUMENTATION: one voice walks a corpus in
//! process and writes what it read. Trace lines only; it changes no
//! behaviour and it is not the fleet.
//!
//! What it measures is the chain L8r opened: a mind extracts a rule over
//! a family with its own epigenome as the crystal (21 step 3), stands
//! what closes, and the next family may cite what the last one stood.
//! The reading that matters is `glued` — rules citing a form this mind
//! holds — which was structurally zero while the glue's read was
//! performed against the form index (26, 27, 50.6).

#[global_allocator]
static ALLOC: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

mod bridge;
mod calibrate;
mod census;
mod cover;
mod confluence;
mod delta;
mod incongruence;
mod interpret;
mod invert;
mod luminal;
mod solve;
mod sporophore;
mod mint;
mod reconstruct;
mod regime;
mod volume;

use mycelium::client::sdk::Epigenome;
use mycelium::transducer::family::{extract_rule_with, mint_sentence};
use mycelium::transducer::sentence::Vertex;

/// What one walk of the corpus read. Every field is a count of acts, read
/// and never branched on (rule 2).
#[derive(Default)]
struct Reading {
    tasks: u64,
    extracted: u64,
    stood: u64,
    glued: u64,
    demands: u64,
    words: u64,
    /// The demand kinds by 50.5's own order — `RotorPrime`, `Standing`,
    /// `Source`, `Creation`, `TopologicalDeformation` — which name what
    /// the alphabet could not spell and the eyes could not pair, and so
    /// which of L10 and L11 a corpus is actually asking for.
    kinds: [u64; 5],
    /// Budget spent across the walk, and the families whose extraction
    /// ended on it rather than on the bundle (L8w).
    spent: u64,
    exhausted: u64,
    /// 45.9 — THE ENGINE'S OWN COORDINATES, which is what the instrument
    /// is for. The kind tally above is the engine rendered into human
    /// buckets and orders nothing; these are the language's own.
    ///
    /// `held` is the generators this mind can factor through — the
    /// distinct `Word` vertices of the forms it holds. `adjoined` is how
    /// many of them the ENGINE opened rather than received: read from
    /// content, a vertex with no citations whose rotor the seventeen
    /// cannot spell (L14 made that representable with no escape byte).
    /// `retired` is what the language shed. `cover_rank` is the strata
    /// the working atlas occupies. `comprehended` is families closed per
    /// generator held, the one ratio that says whether the language is
    /// doing work — carried as its two terms, never as a quotient.
    held: u64,
    adjoined: u64,
    retired: u64,
    cover_rank: u64,
    comprehended: u64,
    /// 50.8 — simplices filled, read from the forms this mind holds exactly
    /// as a court reads them off its ruled cover: a form carrying a
    /// `FiledOrbit` has closed a simplex of its cover.
    simplices_filled: u64,
}

fn main() {
    let argv: Vec<String> = std::env::args().skip(1).collect();
    // Y1 — THE WARM–COLD DELTA, which is the stage's number and is read by
    // `--delta <dir> <limit>`. It walks the slice twice in process and
    // sets its own stroke; the walk below is the level reading and is
    // untouched by it.
    // The flag is at the head of the argument vector because that is where
    // a flag IS on a command line — a wire tag's own offset, rule 15's
    // second case — and the slice pattern reads it there without taking a
    // member by position.
    // Y3's instrumentation (§8.1), `--mint <dir> <limit|name.json>`: trace
    // lines for every task of the slice whose rule form closes, reading what
    // the mint is handed and which of its two calls refuses. It changes
    // nothing; it only prints.
    //
    // A NAME IS READ BY NAME. A row whose observable is one task is proved by
    // reading that task, and asking for it by number means running the sixty
    // in front of it and searching the output — a survey standing in for a
    // keyed read. A third argument that does not parse as a count IS the
    // task's own name, which is the coordinate the corpus files it under.
    // 78.81's caution — `--interpret DIR LIMIT`: the voice interpreter's
    // spend against the extraction's, before a recut.
    if matches!(argv.split_first(), Some((flag, _)) if flag == "--interpret") {
        let Some(dir) = argv.get(1) else {
            eprintln!("  interpret refused: name the corpus directory");
            std::process::exit(1);
        };
        let limit: usize = match argv.get(2).map(|n| n.parse::<usize>()) {
            Some(Ok(n)) => n,
            _ => 60,
        };
        match slice_names(dir, limit) {
            Ok(names) => interpret::report(dir, &names),
            Err(e) => {
                eprintln!("  interpret refused: {e}");
                std::process::exit(1);
            }
        }
        return;
    }
    // 78.88 — `--sporophore DIR LIMIT`: whether reading a scene by its
    // enclosure matters for perception now.
    if matches!(argv.split_first(), Some((flag, _)) if flag == "--sporophore") {
        let Some(dir) = argv.get(1) else {
            eprintln!("  sporophore refused: name the corpus directory");
            std::process::exit(1);
        };
        let limit: usize = match argv.get(2).map(|n| n.parse::<usize>()) {
            Some(Ok(n)) => n,
            _ => 60,
        };
        match slice_names(dir, limit) {
            Ok(names) => sporophore::report(dir, &names),
            Err(e) => {
                eprintln!("  sporophore refused: {e}");
                std::process::exit(1);
            }
        }
        return;
    }
    // 78.84 — `--bridge DIR LIMIT`: the corpus's motions through the law
    // register.
    if matches!(argv.split_first(), Some((flag, _)) if flag == "--bridge") {
        let Some(dir) = argv.get(1) else {
            eprintln!("  bridge refused: name the corpus directory");
            std::process::exit(1);
        };
        let limit: usize = match argv.get(2).map(|n| n.parse::<usize>()) {
            Some(Ok(n)) => n,
            _ => 60,
        };
        match slice_names(dir, limit) {
            Ok(names) => bridge::report(dir, &names),
            Err(e) => {
                eprintln!("  bridge refused: {e}");
                std::process::exit(1);
            }
        }
        return;
    }
    // 78.82 — `--reconstruct DIR LIMIT`: HR1–HR4, reconstruction by holonomy.
    if matches!(argv.split_first(), Some((flag, _)) if flag == "--reconstruct") {
        let Some(dir) = argv.get(1) else {
            eprintln!("  reconstruct refused: name the corpus directory");
            std::process::exit(1);
        };
        let limit: usize = match argv.get(2).map(|n| n.parse::<usize>()) {
            Some(Ok(n)) => n,
            _ => 60,
        };
        match slice_names(dir, limit) {
            Ok(names) => reconstruct::report(dir, &names),
            Err(e) => {
                eprintln!("  reconstruct refused: {e}");
                std::process::exit(1);
            }
        }
        return;
    }
    // 78.80 — `--volume DIR LIMIT`: volume and density against shape.
    if matches!(argv.split_first(), Some((flag, _)) if flag == "--volume") {
        let Some(dir) = argv.get(1) else {
            eprintln!("  volume refused: name the corpus directory");
            std::process::exit(1);
        };
        let limit: usize = match argv.get(2).map(|n| n.parse::<usize>()) {
            Some(Ok(n)) => n,
            _ => 60,
        };
        match slice_names(dir, limit) {
            Ok(names) => volume::report(dir, &names),
            Err(e) => {
                eprintln!("  volume refused: {e}");
                std::process::exit(1);
            }
        }
        return;
    }
    // 78.79 — `--incongruence DIR LIMIT`: the interpreter for incongruence.
    if matches!(argv.split_first(), Some((flag, _)) if flag == "--incongruence") {
        let Some(dir) = argv.get(1) else {
            eprintln!("  incongruence refused: name the corpus directory");
            std::process::exit(1);
        };
        let limit: usize = match argv.get(2).map(|n| n.parse::<usize>()) {
            Some(Ok(n)) => n,
            _ => 60,
        };
        match slice_names(dir, limit) {
            Ok(names) => incongruence::report(dir, &names),
            Err(e) => {
                eprintln!("  incongruence refused: {e}");
                std::process::exit(1);
            }
        }
        return;
    }
    // 78.78 — `--confluence DIR LIMIT`: P1, the remainder's bodies read as
    // creations, single carriages and confluences.
    if matches!(argv.split_first(), Some((flag, _)) if flag == "--confluence") {
        let Some(dir) = argv.get(1) else {
            eprintln!("  confluence refused: name the corpus directory");
            std::process::exit(1);
        };
        let limit: usize = match argv.get(2).map(|n| n.parse::<usize>()) {
            Some(Ok(n)) => n,
            _ => 60,
        };
        match slice_names(dir, limit) {
            Ok(names) => confluence::report(dir, &names),
            Err(e) => {
                eprintln!("  confluence refused: {e}");
                std::process::exit(1);
            }
        }
        return;
    }
    // 78.77 — `--light DIR LIMIT`: L1–L6, what the remainder reads as light.
    if matches!(argv.split_first(), Some((flag, _)) if flag == "--light") {
        let Some(dir) = argv.get(1) else {
            eprintln!("  light refused: name the corpus directory");
            std::process::exit(1);
        };
        let limit: usize = match argv.get(2).map(|n| n.parse::<usize>()) {
            Some(Ok(n)) => n,
            _ => 60,
        };
        match slice_names(dir, limit) {
            Ok(names) => luminal::report(dir, &names),
            Err(e) => {
                eprintln!("  light refused: {e}");
                std::process::exit(1);
            }
        }
        return;
    }
    // 78.76 — `--invert DIR LIMIT`: the remainder inversion measured over a
    // slice — before and after per task, and in total.
    if matches!(argv.split_first(), Some((flag, _)) if flag == "--invert") {
        let Some(dir) = argv.get(1) else {
            eprintln!("  invert refused: name the corpus directory");
            std::process::exit(1);
        };
        let limit: usize = match argv.get(2).map(|n| n.parse::<usize>()) {
            Some(Ok(n)) => n,
            _ => 60,
        };
        let painted = match argv.get(3).map(String::as_str) {
            Some("sites") => mycelium::transducer::family::invert::Painted::Sites,
            _ => mycelium::transducer::family::invert::Painted::Bodies,
        };
        match slice_names(dir, limit) {
            Ok(names) => invert::report(dir, &names, painted),
            Err(e) => {
                eprintln!("  invert refused: {e}");
                std::process::exit(1);
            }
        }
        return;
    }
    if matches!(argv.split_first(), Some((flag, _)) if flag == "--mint") {
        let Some(dir) = argv.get(1) else {
            eprintln!("  mint refused: name the corpus directory");
            std::process::exit(1);
        };
        let named: Option<Vec<String>> = match argv.get(2) {
            Some(a) if a.parse::<usize>().is_err() => Some(vec![a.clone()]),
            _ => None,
        };
        let limit: usize = match argv.get(2).map(|n| n.parse::<usize>()) {
            Some(Ok(n)) => n,
            // A name was given: the slice is that one task and no limit
            // applies to it.
            Some(Err(_)) => 0,
            None => usize::MAX,
        };
        match named {
            Some(names) => mint::trace(dir, &names),
            None => match slice_names(dir, limit) {
                Ok(names) => mint::trace(dir, &names),
                Err(e) => {
                    eprintln!("  mint refused: {e}");
                    std::process::exit(1);
                }
            },
        }
        return;
    }
    // D1's standing reading: a mind stood on a slice, then ONE named
    // family pulled under the voice's own budget, the pull and the mint
    // timed apart. `--regime DIR NAME [SLICE]`, slice 60 by default.
    // K1's reading: what the scene cover deduces on a corpus (53.2).
    // The only question the corpus asks: how many does it solve.
    if matches!(argv.split_first(), Some((flag, _)) if flag == "--solve") {
        let Some(dir) = argv.get(1) else {
            eprintln!("  solve refused: name the corpus directory");
            std::process::exit(1);
        };
        let limit: usize = match argv.get(2).map(|n| n.parse::<usize>()) {
            Some(Ok(n)) => n,
            _ => usize::MAX,
        };
        let stand: usize = match argv.get(3).map(|n| n.parse::<usize>()) {
            Some(Ok(n)) => n,
            _ => 0,
        };
        match slice_names(dir, limit) {
            Ok(names) => solve::report(dir, &names, stand),
            Err(e) => {
                eprintln!("  solve refused: {e}");
                std::process::exit(1);
            }
        }
        return;
    }
    // 53.2's probe — `--census DIR [id,id,…]`: what each family READS as,
    // on the task files the journal's ids map to. It reads the corpus on
    // disk and no fleet number is taken from it.
    if matches!(argv.split_first(), Some((flag, _)) if flag == "--census") {
        let Some(dir) = argv.get(1) else {
            eprintln!("  census refused: name the corpus directory");
            std::process::exit(1);
        };
        // No ids given is the whole corpus, not an empty selection of it:
        // the two are different questions and the match says which.
        let want: Vec<String> = match argv.get(2) {
            Some(a) => a.split(',').map(str::to_string).collect(),
            None => Vec::new(),
        };
        match slice_names(dir, usize::MAX) {
            Ok(names) => census::report(dir, &names, &want),
            Err(e) => {
                eprintln!("  census refused: {e}");
                std::process::exit(1);
            }
        }
        return;
    }
    // Z10's calibration: the walk's arithmetic against the allocator's.
    if matches!(argv.split_first(), Some((flag, _)) if flag == "--find-id") {
        let (Some(dir), Some(want)) = (argv.get(1), argv.get(2)) else {
            eprintln!("  find-id refused: name the corpus directory and the id prefix");
            std::process::exit(1);
        };
        match slice_names(dir, usize::MAX) {
            Ok(names) => calibrate::find_id(dir, &names, want),
            Err(e) => {
                eprintln!("  find-id refused: {e}");
                std::process::exit(1);
            }
        }
        return;
    }
    // BQ3, STD1 — THE ORACLE'S GOLDEN FILE, written on the carrier it
    // stands for: a flag here, never an ignored test (§6.4).
    if matches!(argv.split_first(), Some((flag, _)) if flag == "--write-oracle-golden") {
        match mycelium::transducer::cga::oracle::write_golden() {
            Ok(n) => eprintln!("  oracle: {n} readings written to {}", mycelium::transducer::cga::oracle::GOLDEN),
            Err(e) => {
                eprintln!("  oracle refused: the golden file did not write: {e}");
                std::process::exit(1);
            }
        }
        return;
    }
    if matches!(argv.split_first(), Some((flag, _)) if flag == "--calibrate") {
        let (Some(dir), Some(name)) = (argv.get(1), argv.get(2)) else {
            eprintln!("  calibrate refused: name the corpus directory and the task");
            std::process::exit(1);
        };
        calibrate::report(dir, name);
        return;
    }
    if matches!(argv.split_first(), Some((flag, _)) if flag == "--cover") {
        let Some(dir) = argv.get(1) else {
            eprintln!("  cover refused: name the corpus directory");
            std::process::exit(1);
        };
        let limit: usize = match argv.get(2).map(|n| n.parse::<usize>()) {
            Some(Ok(n)) => n,
            Some(Err(e)) => {
                eprintln!("  cover refused: the task count does not read: {e}");
                std::process::exit(1);
            }
            None => usize::MAX,
        };
        match slice_names(dir, limit) {
            Ok(names) => cover::report(dir, &names),
            Err(e) => {
                eprintln!("  cover refused: {e}");
                std::process::exit(1);
            }
        }
        return;
    }
    if matches!(argv.split_first(), Some((flag, _)) if flag == "--regime") {
        let (Some(dir), Some(name)) = (argv.get(1), argv.get(2)) else {
            eprintln!("  regime refused: name the corpus directory and the task");
            std::process::exit(1);
        };
        let slice: usize = match argv.get(3).map(|n| n.parse::<usize>()) {
            Some(Ok(n)) => n,
            Some(Err(e)) => {
                eprintln!("  regime refused: the slice count does not read: {e}");
                std::process::exit(1);
            }
            None => 60,
        };
        match slice_names(dir, slice) {
            Ok(names) => regime::report(dir, &names, name),
            Err(e) => {
                eprintln!("  regime refused: {e}");
                std::process::exit(1);
            }
        }
        return;
    }
    if matches!(argv.split_first(), Some((flag, _)) if flag == "--delta") {
        let Some(dir) = argv.get(1) else {
            eprintln!("  delta refused: name the corpus directory");
            std::process::exit(1);
        };
        let limit: usize = match argv.get(2).map(|n| n.parse::<usize>()) {
            Some(Ok(n)) => n,
            Some(Err(e)) => {
                eprintln!("  delta refused: the task count does not read: {e}");
                std::process::exit(1);
            }
            None => usize::MAX,
        };
        match slice_names(dir, limit) {
            Ok(names) => delta::report(&delta::walk_twice(dir, &names)),
            Err(e) => {
                eprintln!("  delta refused: {e}");
                std::process::exit(1);
            }
        }
        return;
    }
    let mut args = argv.into_iter();
    // The corpus path and how much of it to walk are the operator's, and
    // an absent argument is an absent argument, never a value this binary
    // invented (rule 11): each arm is written out.
    let dir = match args.next() {
        Some(d) => d,
        None => {
            eprintln!("  soak refused: name the corpus directory");
            std::process::exit(1);
        }
    };
    let limit: usize = match args.next().map(|n| n.parse::<usize>()) {
        Some(Ok(n)) => n,
        Some(Err(e)) => {
            eprintln!("  soak refused: the task count does not read: {e}");
            std::process::exit(1);
        }
        None => usize::MAX,
    };
    // THE STROKE, so a reading can be told apart from a starvation. Every
    // count on this walk falls when the budget ends before the work does,
    // and the only way to know which happened is to run the same corpus at
    // a stroke large enough that nothing ends on it. Absent, the machine's
    // own stroke is used and the walk is what a mind actually gets.
    let stroke: Option<usize> = match args.next().map(|n| n.parse::<usize>()) {
        Some(Ok(n)) => Some(n),
        Some(Err(e)) => {
            eprintln!("  soak refused: the stroke does not read: {e}");
            std::process::exit(1);
        }
        None => None,
    };
    match walk(&dir, limit, stroke) {
        Ok(r) => {
            println!(
                "  soak tasks {} extracted {} stood {} glued {} demands {} words {}",
                r.tasks, r.extracted, r.stood, r.glued, r.demands, r.words
            );
            println!(
                "  soak budget spent {} exhausted {}",
                r.spent, r.exhausted
            );
            println!(
                "  soak demands rotor_prime {} standing {} source {} creation {} deformation {}",
                r.kinds[0], r.kinds[1], r.kinds[2], r.kinds[3], r.kinds[4]
            );
            // The one number that bears on the architecture is `adjoined`.
            // It is zero until 37.7's adjunction runs (L11), and it is
            // printed every walk so that the day it is not zero is a
            // measurement and not an anecdote.
            println!(
                "  soak generators held {} adjoined {} retired {} cover rank {} \
                 families comprehended {} simplices filled {}",
                r.held, r.adjoined, r.retired, r.cover_rank, r.comprehended, r.simplices_filled
            );
        }
        Err(e) => {
            eprintln!("  soak refused: {e}");
            std::process::exit(1);
        }
    }
}

/// THE SLICE: the corpus directory's files in name order, truncated to
/// the operator's limit. One carrier for it (law 12), so the level walk
/// and Y1's delta read the same slice in the same order and a reading of
/// one can be set beside a reading of the other.
fn slice_names(dir: &str, limit: usize) -> Result<Vec<String>, String> {
    let mut names: Vec<String> = Vec::new();
    let entries = std::fs::read_dir(dir).map_err(|e| format!("{dir}: {e}"))?;
    for e in entries {
        let e = e.map_err(|e| format!("{dir}: {e}"))?;
        if let Some(n) = e.file_name().to_str() {
            names.push(n.to_string());
        }
    }
    names.sort();
    names.truncate(limit);
    Ok(names)
}

/// The corpus is the boundary this loop walks (rule 10): one file, one
/// task, in name order so two runs read the same corpus in the same
/// order.
fn walk(dir: &str, limit: usize, stroke: Option<usize>) -> Result<Reading, String> {
    let names = slice_names(dir, limit)?;
    let mut epi = Epigenome::new();
    let mut physics = mycelium::physics::Physics::default();
    let mut r = Reading::default();
    let mut k = 0u64;
    for name in &names {
        let path = format!("{dir}/{name}");
        let Ok(json) = std::fs::read_to_string(&path) else {
            continue;
        };
        let Some(task) = mycelium::arc::parse_task(&json) else {
            continue;
        };
        r.tasks += 1;
        k += 1;
        // One line per task before the extraction, so a family that does
        // not return is named by the last line written rather than by a
        // guess (rule 10: a loop whose bound is neither a Budget nor a
        // boundary is unbounded work, and this is how it is found).
        eprintln!("  soak task {name} pairs {}", task.train.len());
        // 21 with the mind as its own crystal: what it stood on an earlier
        // family is what it may cite on this one.
        // L8w: the extraction runs under a stroke like every other read,
        // and what it has when the budget ends is a withheld form. The
        // budget spent is itself a reading — a family that exhausts it is
        // one the search could not finish, and 50.2 step 2 is why.
        let mut budget = match stroke {
            Some(n) => physics.stroke_n(n),
            None => physics.stroke(),
        };
        let before = budget.remaining();
        let Some(rule) = extract_rule_with(&task.train, &epi, &mut budget) else {
            continue;
        };
        // Exact, not saturating (rule 11): a budget's remaining reads only
        // ever fall, so `before` is never below it and the difference is
        // the spend itself.
        r.spent += (before - budget.remaining()) as u64;
        if budget.remaining() == 0 {
            r.exhausted += 1;
        }
        r.extracted += 1;
        for (_, v) in rule.form.vertices.scoped() {
            match v {
                Vertex::Glued { .. } => r.glued += 1,
                Vertex::Demand { kind, .. } => {
                    r.demands += 1;
                    if let Some(slot) = r.kinds.get_mut(usize::from(kind.kind_byte())) {
                        *slot += 1;
                    }
                }
                Vertex::Word { .. } => r.words += 1,
                _ => {}
            }
        }
        // Stand what closed, so the next family can cite it (24.3).
        let Some(spoken) = mint_sentence(&rule, &task.train, &epi, 0) else {
            continue;
        };
        // Only a closed form is citable (13.6, `Form::glue`), so only a
        // closed form is stood here for the next family to reach. The
        // decode is `Held::of`'s, which is the same decode the dialect
        // performs when it stands the sentence a line below (L16).
        let held = mycelium::transducer::element::Held::of(spoken);
        let Some(form) = held.form() else {
            continue;
        };
        let Some(()) = form.is_closed().then_some(()) else {
            continue;
        };
        if epi.stand_dialect_at(held.element().clone(), k, k).is_some() {
            r.stood += 1;
            r.comprehended += 1;
        }
    }
    read_engine(&mut epi, &mut r);
    Ok(r)
}

/// 45.9: the engine's coordinates, read off the mind at the end of the
/// walk. Every one is read from CONTENT — the forms the mind holds and
/// the vertices in them — and nothing is counted that was not derived.
fn read_engine(epi: &mut Epigenome, r: &mut Reading) {
    let mut seen: mycelium::network::nerve::Keyed<()> = mycelium::network::nerve::Keyed::new();
    let mut charts: mycelium::network::nerve::Keyed<()> = mycelium::network::nerve::Keyed::new();
    for e in epi.genome_words() {
        let Some(f) = e.form() else {
            continue;
        };
        charts.insert(mycelium::transducer::stalk::invariant_of(e).key(), ());
        // One form's torsion carrier, and one genome's charts below: both
        // are fibers already, the first by the form that holds it and the
        // second by being built here.
        r.simplices_filled += f.torsion.scoped().count() as u64;
        for (_, v) in f.vertices.scoped() {
            let Vertex::Word { address, derivation, .. } = v else {
                continue;
            };
            if seen.contains(address.as_bytes()) {
                continue;
            }
            seen.insert(address.clone(), ());
            r.held += 1;
            // ADJOINED, READ FROM CONTENT (37.7): a generator the engine
            // opened is one that cites nothing and whose action the
            // seventeen cannot spell. A vertex that spells is a word of
            // the alphabet however it was reached; a vertex that cites is
            // a composition of what was already there.
            let spelled = mycelium::transducer::alphabet::factor_rotor(&derivation.rotor());
            let unspellable = !matches!(spelled, Some(Ok(_)));
            if derivation.generators.is_empty() && unspellable {
                r.adjoined += 1;
            }
        }
    }
    r.cover_rank = charts.scoped().count() as u64;
    r.retired = epi.take_promoted().len() as u64;
}
