// AI-GENERATED CONTENT — see DISCLAIMER.md.
mod lint;

// The warning gate is declared HERE and not in `lint/mod.rs`, because
// `build.rs` includes that module and would carry an item it never calls.
// Only this binary runs clippy; see `lint/warnings.rs`.
#[path = "lint/warnings.rs"]
mod warnings;

fn main() {
    let refuse = |e: String| -> ! {
        eprintln!("{e}");
        std::process::exit(1)
    };
    // The file gates: what `build.rs` also runs, on every build.
    if let Err(e) = lint::run() {
        refuse(e);
    }
    // The warning gate: clippy over every target with warnings denied.
    // `build.rs` cannot run this one — see `lint/warnings.rs` — so the
    // xtask binary is where it lives. `xtask gates` asks for the file gates
    // alone, for a caller that wants the fast half.
    if std::env::args().any(|a| a == "gates") {
        return;
    }
    let manifest = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let crate_dir = if manifest.join("src/server").is_dir() {
        manifest
    } else {
        match manifest.parent() {
            Some(p) => p.to_path_buf(),
            None => refuse("xtask has no parent".to_string()),
        }
    };
    if let Err(e) = warnings::run(&crate_dir) {
        refuse(e);
    }
}
