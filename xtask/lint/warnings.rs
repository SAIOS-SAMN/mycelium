// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! MU-STANDARDS 4 — THE WARNING GATE: the tree compiles and lints CLEAN,
//! or the gate refuses. `cargo clippy --release --all-targets -- -D
//! warnings` — every target, tests included, rustc's warnings denied too.

use std::process::Command;

/// Run clippy over every target from `crate_dir`. This gate runs from the
/// `xtask` binary alone and never from `build.rs`: a build script runs
/// DURING a build, and invoking cargo from inside one re-enters the build
/// it is part of.
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
