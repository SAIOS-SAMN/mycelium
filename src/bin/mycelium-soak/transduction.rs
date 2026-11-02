// O10 — THE TRANSDUCTION CENSUS: every train pair of unequal extent in the
// corpus, classed by how its two frames relate.
//
//! Is every pair relation a product of a signed permutation of the phases,
//! the re-period of each phase, an offset and a re-enclosure
//! (MU-DERIVATIONS D3.2 O10)? Each unequal-extent train pair is read by
//! `transduce::relation_of` and counted under the class it reads as: a typed
//! reading of the ARC files on disk, never a mesh number and never a log
//! grep. The residue (`Unenclosed`) is the part no such product carries,
//! and its count is the answer's size.

use mycelium::transducer::family::transduce::{relation_of, PairRelation, Sense, SignedPermutation};

/// THE CENSUS, counted by class. A class is the relation's own reading, so
/// a permutation is named by its images and a cover by its counts. The
/// tallies are by position in the rank-2 group as generated.
#[derive(Default)]
struct RelationTally {
    pairs: u64,
    rank: usize,
    carried: Vec<u64>,
    covers: u64,
    offset: Vec<u64>,
    reenclosed: Vec<u64>,
    unenclosed: u64,
    refused: u64,
}

/// A signed permutation named by its images: output phase `j` as `±k`,
/// the source phase `k` and its sense.
fn name(s: &SignedPermutation) -> String {
    let images: Vec<String> = s
        .images()
        .iter()
        .map(|&(k, sense)| match sense {
            Sense::Kept => format!("+{k}"),
            Sense::Reversed => format!("-{k}"),
        })
        .collect();
    format!("({})", images.join(" "))
}

/// The permutation's position in `SignedPermutation::all`, the group's own
/// order.
fn slot(s: &SignedPermutation) -> usize {
    match SignedPermutation::all(s.rank()).position(|t| t == *s) {
        Some(i) => i,
        None => 0,
    }
}

/// Count `s` in `row`, grown to the slot it names.
fn count(row: &mut Vec<u64>, s: &SignedPermutation) {
    let i = slot(s);
    if row.len() <= i {
        row.resize(i + 1, 0);
    }
    if let Some(n) = row.get_mut(i) {
        *n += 1;
    }
}

/// The census over the files `names` of `dir`.
pub(crate) fn report(dir: &str, names: &[String]) {
    let mut tally = RelationTally::default();
    for file in names {
        let Ok(json) = std::fs::read_to_string(format!("{dir}/{file}")) else { continue };
        let Some(task) = mycelium::arc::parse_task(&json) else { continue };
        let ground = mycelium::arc::ground_bytes();
        let ground = mycelium::transducer::sample::Symbol::new(&ground);
        for (x, y) in &task.train {
            let extent = |g: &mycelium::arc::Grid| (g.len(), g.iter().next().map(Vec::len));
            if extent(x) == extent(y) {
                continue;
            }
            tally.pairs += 1;
            let (Ok(a), Ok(b)) = (mycelium::arc::sample_unplaced(x), mycelium::arc::sample_unplaced(y)) else {
                tally.refused += 1;
                continue;
            };
            tally.rank = a.rank();
            match relation_of(&a, &b, ground) {
                Ok(PairRelation::Carried(s)) => count(&mut tally.carried, &s),
                Ok(PairRelation::Cover { .. }) => tally.covers += 1,
                Ok(PairRelation::Offset(s)) => count(&mut tally.offset, &s),
                Ok(PairRelation::Reenclosed(s)) => count(&mut tally.reenclosed, &s),
                Ok(PairRelation::Unenclosed) => tally.unenclosed += 1,
                Err(_) => tally.refused += 1,
            }
        }
    }
    // The group of the rank the samples were read at, as generated.
    let names: Vec<String> = SignedPermutation::all(tally.rank).map(|s| name(&s)).collect();
    println!("  transduction census over {dir}: {} unequal-extent train pairs", tally.pairs);
    for (label, row) in [("carried", &tally.carried), ("offset", &tally.offset), ("re-enclosed", &tally.reenclosed)] {
        let total: u64 = row.iter().sum();
        let by: Vec<String> = names.iter().zip(row.iter()).filter(|(_, &n)| n > 0).map(|(s, n)| format!("{s} {n}")).collect();
        println!("  {label:<12} {total:>5}  ({})", by.join(", "));
    }
    println!("  {:<12} {:>5}  (refused Seam as one map; nested enclosures)", "cover", tally.covers);
    println!("  {:<12} {:>5}", "unenclosed", tally.unenclosed);
    println!("  {:<12} {:>5}  (no enclosure: empty or ragged)", "refused", tally.refused);
}
