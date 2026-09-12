// MU-STANDARDS 4.1, 6.1 — THE GATE BINARY: the reading bound, then
// clippy with warnings denied. Non-zero on either.

mod lint;

#[path = "lint/warnings.rs"]
mod warnings;

fn main() {
    let refuse = |e: String| -> ! {
        eprintln!("{e}");
        std::process::exit(1)
    };
    if let Err(e) = lint::run() {
        refuse(e);
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
