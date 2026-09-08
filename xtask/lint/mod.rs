// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! THE GATES. One file per gate (Stage 8b S29), each named for the rule it
//! enforces: `length.rs` the reading bound, `enumeration.rs` rules 1 and 8,
//! `maps.rs` and `digest.rs` law 2, `matrix.rs` and `squaring.rs` law 11,
//! `clock.rs` law 9. This file is the order they run in and nothing else.

pub mod clock;
pub mod digest;
pub mod walker;
pub mod enumeration;
pub mod length;
pub mod maps;
pub mod matrix;
pub mod order;
pub mod squaring;

use length::lint_file_length;

use std::path::PathBuf;

pub fn run() -> Result<(), String> {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let root = if manifest.join("src/server").is_dir() {
        manifest.join("src")
    } else {
        manifest
            .parent()
            .ok_or("xtask has no parent")?
            .join("src")
    };
    let mut hits = Vec::new();
    // The reading bound covers the gate itself: `lint.rs` is a source file a
    // reviewer must read whole like any other (MU-STANDARDS 1.1), so `xtask`
    // is walked beside `src`. Both in ONE call: the stale-row check below is
    // over everything the bound walks, and two calls would each report the
    // other's rows as missing.
    let bounded: Vec<PathBuf> = match root.parent() {
        Some(up) => vec![root.clone(), up.join("xtask")],
        None => vec![root.clone()],
    };
    lint_file_length(&bounded, &mut hits)?;
    // ONE WALK over the tree the rules govern, every gate, allow-lists by
    // file (4.2, 4.5). The scope is `src/` and not `xtask/`: the content
    // rules are about cognition, and a gate that forbids a pattern must
    // contain that pattern to search for it. The reading bound above is the
    // one rule that does cover `xtask/`, because it is about reading.
    let mut files = Vec::new();
    walker::collect(&root, &mut files)?;
    walker::run_gates(&files, &[
            clock::GATE,
            digest::GATE,
            enumeration::GATE,
            maps::GATE,
            matrix::GATE,
            order::GATE,
            squaring::GATE,
        ], &mut hits)?;
    if hits.is_empty() {
        return Ok(());
    }
    let mut msg = String::from("mycelium gates refused this tree:\n");
    for h in &hits {
        msg.push_str(h);
        msg.push('\n');
    }
    Err(msg)
}

