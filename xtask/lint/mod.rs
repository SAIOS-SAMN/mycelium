// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! THE GATES. One file per gate (Stage 8b S29), each named for the rule it
//! enforces: `length.rs` the reading bound, `enumeration.rs` rules 1 and 8,
//! `maps.rs` and `digest.rs` law 2, `matrix.rs` and `squaring.rs` law 11,
//! `clock.rs` law 9. This file is the order they run in and nothing else.

pub mod clock;
pub mod digest;
pub mod enumeration;
pub mod length;
pub mod maps;
pub mod matrix;
pub mod squaring;

use clock::lint_clock;
use digest::lint_blake3;
use enumeration::walk;
use length::lint_file_length;
use maps::lint_maps;
use matrix::lint_qmatrix;
use squaring::lint_squaring;

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
    let dirs = ["server", "client", "transducer", "network"];
    let mut hits = Vec::new();
    for d in dirs {
        walk(&root.join(d), d, &mut hits)?;
    }
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
    lint_blake3(&root, &mut hits)?;
    lint_squaring(&root, &mut hits)?;
    lint_maps(&root, &mut hits)?;
    lint_qmatrix(&root, &mut hits)?;
    lint_clock(&root, &mut hits)?;
    if hits.is_empty() {
        return Ok(());
    }
    let mut msg = String::from("lint-enumeration: a loop without a keyed source:\n");
    for h in &hits {
        msg.push_str(h);
        msg.push('\n');
    }
    Err(msg)
}

