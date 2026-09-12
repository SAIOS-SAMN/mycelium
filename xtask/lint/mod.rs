// MU-STANDARDS 4.1 — THE READING BOUND, over `src/` and `xtask/`. Every
// other rule the compiler can read is a lint (§4.2) or a type (§4.4).

pub mod length;

use length::lint_file_length;

use std::path::PathBuf;

pub fn run() -> Result<(), String> {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let root = if manifest.join("src/server").is_dir() {
        manifest.join("src")
    } else {
        manifest.parent().ok_or("xtask has no parent")?.join("src")
    };
    // The bound is about reading, so it covers the gate binary too.
    let bounded: Vec<PathBuf> = match root.parent() {
        Some(up) => vec![root.clone(), up.join("xtask")],
        None => vec![root.clone()],
    };
    let mut hits = Vec::new();
    lint_file_length(&bounded, &mut hits)?;
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
