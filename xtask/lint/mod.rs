// MU-STANDARDS 4.1 — THE GATES THE COMPILER CANNOT READ, over `src/`,
// `forms/src/`, `sdk/src/` and `xtask/`: the reading bound (§4.1), the Euclidean grid vocabulary
// (law 2, rule 15, the Anti-Flattening Law) and the numbers gate (MU-STANDARDS
// 2.12, MU-DERIVATIONS D1.4). Every other rule is a clippy lint (§4.2) or a
// type (§4.4).

pub mod euclidean;
pub mod length;
pub mod numbers;

#[cfg(test)]
mod tests;

use euclidean::lint_euclidean;
use length::lint_file_length;
use numbers::lint_numbers;

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
    // NO CONSTANT: every literal-bearing numeric item is classified, and
    // every underived one is named in the register (N1–N5). N6, the inline
    // literals of the law paths, is reported and refuses nothing yet.
    if let Some(repo) = root.parent() {
        let numbers = lint_numbers(repo)?;
        eprintln!(
            "numbers: {} classified numeric items; N6 inventory {} inline literals in the law paths (reported, not enforced; `gate inventory` lists them)",
            numbers.items,
            numbers.inventory.len()
        );
        hits.extend(numbers.hits);
    }
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
