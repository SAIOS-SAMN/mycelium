// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! THE WARNING GATE: the tree compiles and lints CLEAN, or the gate refuses.
//! `cargo clippy --release --all-targets -- -D warnings` — every target,
//! tests included, with rustc's own warnings denied by the same flag.
//!
//! This gate does NOT run from `build.rs`. A build script runs DURING a
//! build; invoking cargo from inside one re-enters the build it is part of.
//! So the file gates run in both places (they only read source) and this one
//! runs from the `xtask` binary alone, which is where a person or a script
//! asks for it. `xtask/main.rs` declares this module directly rather than
//! `lint/mod.rs`, so the build script does not carry an item it never calls.

use std::process::Command;

pub fn run(crate_dir: &std::path::Path) -> Result<(), String> {
    let status = Command::new("cargo")
        .args(["clippy", "--release", "--all-targets", "--", "-D", "warnings"])
        .current_dir(crate_dir)
        .status()
        .map_err(|e| format!("the warning gate could not run cargo: {e}"))?;
    if status.success() {
        return Ok(());
    }
    Err(format!(
        "mycelium gates refused this tree:\n  \
         `cargo clippy --release --all-targets -- -D warnings` exited {}\n  \
         the rows above are the refusal; a warning is a defect with a line number",
        status.code().map(|c| c.to_string()).unwrap_or_else(|| "by signal".into())
    ))
}
