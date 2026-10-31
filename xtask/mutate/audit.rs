use super::bench::Bench;
use super::catalogue::CATALOGUE;
use super::refusal::MutateRefusal;
use super::verdict::{line, summary};
use super::worktree::Worktree;
use std::path::{Path, PathBuf};

/// The repository root the gate binary was built from.
pub(super) fn repo_root() -> Result<PathBuf, MutateRefusal> {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    if manifest.join("src/server").is_dir() {
        return Ok(manifest);
    }
    manifest.parent().map(Path::to_path_buf).ok_or(MutateRefusal::NoRepo)
}

/// Run the whole catalogue. `Ok(true)` when every mutant was killed.
pub fn run() -> Result<bool, MutateRefusal> {
    let repo = repo_root()?;
    let target_dir = repo.join("target/mutate");
    let tree = Worktree::add(&repo, &target_dir.join("worktree"))?;
    let bench = Bench { tree: tree.path.clone(), target_dir };
    bench.baseline(CATALOGUE)?;
    let mut verdicts = Vec::with_capacity(CATALOGUE.len());
    for (i, m) in CATALOGUE.iter().enumerate() {
        eprintln!("mutate: {}/{} {} [{}]", i + 1, CATALOGUE.len(), m.law, m.file);
        let v = bench.apply(m)?;
        println!("{}", line(m, &v));
        verdicts.push(v);
    }
    drop(tree);
    let (text, green) = summary(&verdicts);
    println!("{text}");
    Ok(green)
}
