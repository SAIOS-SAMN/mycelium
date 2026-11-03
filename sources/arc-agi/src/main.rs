// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! `mycelium-arc-agi decode FROM TO [LIMIT]` — the ARC-AGI-2 corpus posted
//! (MU-SENSING §7; ROADMAP track 26, S1): each task file `<name>.json` of
//! the split directory FROM, in name order and up to LIMIT, read by the
//! corpus's decoder (its frame fixed where it is a gauge) and written to TO
//! as the posted task `<name>.task` that a hypha's corpus directory, a
//! client's feed and the soak read. A file that does not decode is said and
//! skipped.

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let (from, to, limit) = match args.as_slice() {
        [verb, from, to] if verb == "decode" => (from, to, usize::MAX),
        [verb, from, to, n] if verb == "decode" => match n.parse::<usize>() {
            Ok(n) => (from, to, n),
            Err(e) => {
                eprintln!("  decode refused: the limit does not read: {e}");
                std::process::exit(2);
            }
        },
        _ => {
            eprintln!("usage: mycelium-arc-agi decode FROM TO [LIMIT]");
            std::process::exit(2);
        }
    };
    match mycelium_source_arc_agi::post(from, std::path::Path::new(to), limit) {
        Ok(posted) => {
            for (name, why) in &posted.refused {
                eprintln!("  {name} {why}: skipped");
            }
            println!("decoded {} tasks from {from} into {to} ({} refused)", posted.written, posted.refused.len());
        }
        Err(e) => {
            eprintln!("  decode refused: {e}");
            std::process::exit(1);
        }
    }
}
