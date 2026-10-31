// O10 — THE TRANSDUCTION CENSUS: every train pair of unequal extent in the
// corpus, classed by how its two frames relate.
//
//! Is every pair relation a product of a signed swap, the re-period of each
//! circle, an offset and a re-enclosure (MU-DERIVATIONS D3.2 O10)? Each
//! unequal-extent train pair is read by `transduce::relation_of` and counted
//! under the class it reads as: a typed reading of the ARC files on disk,
//! never a mesh number and never a log grep. The residue (`Unenclosed`) is
//! the part no such product carries, and its count is the answer's size.

use mycelium::transducer::family::transduce::{relation_of, PairRelation, SignedSwap};

/// THE CENSUS, counted by class. A class is the relation's own reading, so
/// a swap is named by its three coordinates and a cover by its two counts.
#[derive(Default)]
struct RelationTally {
    pairs: u64,
    carried: [u64; 8],
    covers: u64,
    offset: [u64; 8],
    reenclosed: [u64; 8],
    unenclosed: u64,
    refused: u64,
}

/// The swap's position in `SignedSwap::ALL`, the group's own order.
fn slot(s: SignedSwap) -> usize {
    match SignedSwap::ALL.iter().position(|t| *t == s) {
        Some(i) => i,
        None => 0,
    }
}

/// The census over the files `names` of `dir`.
pub(crate) fn report(dir: &str, names: &[String]) {
    let mut tally = RelationTally::default();
    for file in names {
        let Ok(json) = std::fs::read_to_string(format!("{dir}/{file}")) else { continue };
        let Some(task) = mycelium::arc::parse_task(&json) else { continue };
        for (x, y) in &task.train {
            let extent = |g: &mycelium::arc::Grid| (g.len(), g.iter().next().map(Vec::len));
            if extent(x) == extent(y) {
                continue;
            }
            tally.pairs += 1;
            match relation_of(x, y) {
                Ok(PairRelation::Carried(s)) => tally.carried[slot(s)] += 1,
                Ok(PairRelation::Cover { .. }) => tally.covers += 1,
                Ok(PairRelation::Offset(s)) => tally.offset[slot(s)] += 1,
                Ok(PairRelation::Reenclosed(s)) => tally.reenclosed[slot(s)] += 1,
                Ok(PairRelation::Unenclosed) => tally.unenclosed += 1,
                Err(_) => tally.refused += 1,
            }
        }
    }
    let names = ["identity", "flip cols", "flip rows", "180°", "transpose", "270°", "90°", "anti-transpose"];
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
