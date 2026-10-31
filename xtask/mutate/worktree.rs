use super::bench::tail;
use super::refusal::MutateRefusal;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A scratch worktree, removed when dropped, whatever path leaves the audit.
pub(super) struct Worktree {
    repo: PathBuf,
    pub(super) path: PathBuf,
}

impl Worktree {
    pub(super) fn add(repo: &Path, path: &Path) -> Result<Worktree, MutateRefusal> {
        // A worktree left by an interrupted audit is removed first.
        if path.exists() {
            remove_worktree(repo, path);
        }
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| MutateRefusal::Io { path: parent.to_path_buf(), detail: e.to_string() })?;
        }
        let out = Command::new("git")
            .arg("-C")
            .arg(repo)
            .args(["worktree", "add", "--detach"])
            .arg(path)
            .arg("HEAD")
            .output()
            .map_err(|e| MutateRefusal::Git { step: "worktree add", detail: e.to_string() })?;
        if !out.status.success() {
            return Err(MutateRefusal::Git { step: "worktree add", detail: tail(&out.stderr, 8) });
        }
        let tree = Worktree { repo: repo.to_path_buf(), path: path.to_path_buf() };
        // The corpus is untracked: a suite that reads it reads the repository's.
        let data = repo.join("data");
        if data.exists() {
            symlink_dir(&data, &tree.path.join("data"))?;
        }
        Ok(tree)
    }
}

impl Drop for Worktree {
    fn drop(&mut self) {
        remove_worktree(&self.repo, &self.path);
    }
}

fn remove_worktree(repo: &Path, path: &Path) {
    let removed = Command::new("git")
        .arg("-C")
        .arg(repo)
        .args(["worktree", "remove", "--force"])
        .arg(path)
        .output();
    match removed {
        Ok(out) if out.status.success() => {}
        Ok(out) => eprintln!("mutate: git worktree remove {}: {}", path.display(), tail(&out.stderr, 4)),
        Err(e) => eprintln!("mutate: git worktree remove {}: {e}", path.display()),
    }
    if path.exists() {
        if let Err(e) = std::fs::remove_dir_all(path) {
            eprintln!("mutate: {} could not be removed: {e}", path.display());
        }
    }
    let _ = Command::new("git").arg("-C").arg(repo).args(["worktree", "prune"]).output();
}

#[cfg(unix)]
fn symlink_dir(from: &Path, to: &Path) -> Result<(), MutateRefusal> {
    std::os::unix::fs::symlink(from, to).map_err(|e| MutateRefusal::Io { path: to.to_path_buf(), detail: e.to_string() })
}

#[cfg(not(unix))]
fn symlink_dir(_from: &Path, to: &Path) -> Result<(), MutateRefusal> {
    Err(MutateRefusal::Io { path: to.to_path_buf(), detail: "a corpus symlink needs a unix host".to_string() })
}
