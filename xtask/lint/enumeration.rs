// Rules 1 and 8 — ENUMERATION: a loop over a carrier reads a keyed source,
// or it is a search.
//
//! `.entries()`, `.for_each(` and `.for_each_from(` walk a carrier whole;
//! the move is a keyed read. `spend_unit()` names the spend count as a
//! number: `physics.rs` defines it and `network/wire.rs` reads it, because
//! the wire MTU IS the byte size of one spend.

use super::walker::Gate;
use std::path::Path;

pub(crate) const GATE: Gate = Gate {
    name: "enumeration",
    allow: &[],
    scan,
};

fn scan(path: &Path, src: &str, hits: &mut Vec<String>) {
    let p = path.to_string_lossy().replace('\\', "/");
    // `spend_unit()` names the spend count as a number. Two files may:
    // `physics.rs`, which DEFINES it and is where the number lives, and
    // `network/wire.rs`, because the wire MTU is the byte size of one spend.
    // One pattern of the gate, not the whole gate, so it is a condition
    // here rather than an allow row. `physics.rs` is new to this gate: the
    // old scope was four directories and the crate root was outside it.
    let allow_spend = p.ends_with("network/wire.rs") || p.ends_with("src/physics.rs");
    // FIVE PATTERNS SCOPED TO ONE DIRECTORY, inherited. `.position(`,
    // `.find(`, `.find_map(`, `Vec<bool>` and `unwrap_or` are rules 1, 8, 11
    // and 15, which hold everywhere, and the gate applies them under
    // `sentence/` alone. Widening them is measured at 119 files —
    // `unwrap_or` in 82 of them — and is its own item, not this migration.
    let sentence_only = p.contains("/sentence/");
    for (i, line) in src.lines().enumerate() {
        let mut check = |pat: &str| {
            if line.contains(pat) {
                hits.push(format!("{}:{}: {pat} (rules 1, 8)", path.display(), i + 1));
            }
        };
        check(".entries()");
        check(".for_each(");
        check(".for_each_from(");
        if !allow_spend {
            check("spend_unit()");
        }
        if sentence_only {
            check(".position(");
            check(".find(");
            check(".find_map(");
            check("Vec<bool>");
            check("unwrap_or");
        }
    }
}
