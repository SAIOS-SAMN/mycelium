// MU-STANDARDS 4.1 — THE GATES THE COMPILER CANNOT READ, over `src/`,
// `forms/src/`, `sdk/src/` and `xtask/`: the reading bound (§4.1) and the Euclidean grid vocabulary
// (law 2, rule 15, the Anti-Flattening Law). Every other rule is a clippy
// lint (§4.2) or a type (§4.4).

pub mod euclidean;
pub mod length;

#[cfg(test)]
mod tests;

use euclidean::lint_euclidean;
use length::lint_file_length;

use std::path::PathBuf;

pub fn run() -> Result<(), String> {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let root = if manifest.join("src/server").is_dir() {
        manifest.join("src")
    } else {
        manifest.parent().ok_or("xtask has no parent")?.join("src")
    };
    // The bound is about reading, so it covers the gate binary and every
    // workspace member's source too.
    let bounded: Vec<PathBuf> = match root.parent() {
        Some(up) => vec![root.clone(), up.join("forms/src"), up.join("sdk/src"), up.join("xtask")],
        None => vec![root.clone()],
    };
    let mut hits = Vec::new();
    lint_file_length(&bounded, &mut hits)?;
    // The manifold is not a grid: no Morton curve, no bounding box, no
    // subdivision index standing in for a coordinate (law 2, rule 15,
    // the Anti-Flattening Law).
    lint_euclidean(&bounded, &mut hits)?;
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
