// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! THE MUTATION AUDIT: a law whose breakage no test notices is unguarded.
//!
//! Each mutant of [`catalogue::CATALOGUE`] is applied, one at a time, by
//! exact string replacement inside a scratch git worktree at HEAD — never
//! the working tree — and its narrowest suite is run there. The suite must
//! FAIL. The file is restored before the next mutant, and the worktree is
//! removed at the end, on every path out.
//!
//! Before any mutant, every distinct suite is run on the unmutated tree: a
//! suite red or empty at HEAD proves nothing by failing, and the audit is
//! refused by name rather than read.

pub mod catalogue;

use catalogue::{Mutant, CATALOGUE};
use std::fmt;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

/// Why the audit could not be taken at all.
#[derive(Debug)]
pub enum MutateError {
    /// The repository root could not be read off the xtask manifest.
    NoRepo,
    /// A git step failed.
    Git { step: &'static str, detail: String },
    /// A file could not be read or written.
    Io { path: PathBuf, detail: String },
    /// cargo could not be started.
    Cargo { detail: String },
    /// A suite does not build on the unmutated tree.
    BaselineUnbuilt { suite: String, detail: String },
    /// A suite fails on the unmutated tree.
    BaselineRed { suite: String, failing: Vec<String> },
    /// A suite runs no test on the unmutated tree.
    BaselineEmpty { suite: String },
}

impl fmt::Display for MutateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MutateError::NoRepo => write!(f, "mutate refused: the xtask manifest has no parent repository"),
            MutateError::Git { step, detail } => write!(f, "mutate refused: git {step} failed: {detail}"),
            MutateError::Io { path, detail } => write!(f, "mutate refused: {}: {detail}", path.display()),
            MutateError::Cargo { detail } => write!(f, "mutate refused: cargo could not run: {detail}"),
            MutateError::BaselineUnbuilt { suite, detail } => {
                write!(f, "mutate refused: suite `{suite}` does not build at HEAD:\n{detail}")
            }
            MutateError::BaselineRed { suite, failing } => {
                write!(f, "mutate refused: suite `{suite}` fails at HEAD: {}", failing.join(", "))
            }
            MutateError::BaselineEmpty { suite } => {
                write!(f, "mutate refused: suite `{suite}` runs no test at HEAD")
            }
        }
    }
}

/// What one mutant's suite said.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Verdict {
    /// The suite failed: the law is guarded, by these tests.
    Killed(Vec<String>),
    /// The suite passed with the law broken: a guard gap.
    Survived,
    /// The `before` span was not found exactly once.
    Stale(String),
    /// The mutated tree did not compile: the guard is not proven.
    Error(String),
}

impl Verdict {
    fn word(&self) -> &'static str {
        match self {
            Verdict::Killed(_) => "KILLED",
            Verdict::Survived => "SURVIVED",
            Verdict::Stale(_) => "STALE",
            Verdict::Error(_) => "ERROR",
        }
    }

    /// Whether this verdict leaves its law unproven.
    fn unproven(&self) -> bool {
        !matches!(self, Verdict::Killed(_))
    }
}

/// A scratch worktree, removed when dropped, whatever path leaves the audit.
struct Worktree {
    repo: PathBuf,
    path: PathBuf,
}

impl Worktree {
    fn add(repo: &Path, path: &Path) -> Result<Worktree, MutateError> {
        // A worktree left by an interrupted audit is removed first.
        if path.exists() {
            remove_worktree(repo, path);
        }
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| MutateError::Io { path: parent.to_path_buf(), detail: e.to_string() })?;
        }
        let out = Command::new("git")
            .arg("-C")
            .arg(repo)
            .args(["worktree", "add", "--detach"])
            .arg(path)
            .arg("HEAD")
            .output()
            .map_err(|e| MutateError::Git { step: "worktree add", detail: e.to_string() })?;
        if !out.status.success() {
            return Err(MutateError::Git { step: "worktree add", detail: tail(&out.stderr, 8) });
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
fn symlink_dir(from: &Path, to: &Path) -> Result<(), MutateError> {
    std::os::unix::fs::symlink(from, to).map_err(|e| MutateError::Io { path: to.to_path_buf(), detail: e.to_string() })
}

#[cfg(not(unix))]
fn symlink_dir(_from: &Path, to: &Path) -> Result<(), MutateError> {
    Err(MutateError::Io { path: to.to_path_buf(), detail: "a corpus symlink needs a unix host".to_string() })
}

/// The last `n` lines of a process stream, for a refusal's detail.
fn tail(bytes: &[u8], n: usize) -> String {
    let text = String::from_utf8_lossy(bytes);
    let lines: Vec<&str> = text.lines().collect();
    lines[lines.len().saturating_sub(n)..].join("\n")
}

/// The compiler's error lines, for an ERROR verdict.
fn compile_errors(bytes: &[u8]) -> String {
    let text = String::from_utf8_lossy(bytes);
    let errors: Vec<&str> = text.lines().filter(|l| l.starts_with("error")).take(4).collect();
    match errors.is_empty() {
        true => tail(bytes, 4),
        false => errors.join(" | "),
    }
}

/// What a test run printed: the tests that failed and how many passed.
fn read_run(out: &Output) -> (Vec<String>, usize) {
    let text = String::from_utf8_lossy(&out.stdout);
    let mut failing = Vec::new();
    let mut passed = 0usize;
    for line in text.lines() {
        let Some(rest) = line.trim().strip_prefix("test ") else { continue };
        if let Some(name) = rest.strip_suffix(" ... FAILED") {
            failing.push(name.to_string());
        } else if rest.ends_with(" ... ok") {
            passed += 1;
        }
    }
    (failing, passed)
}

/// A suite: its target selection and filter, as cargo is given them.
fn suite_name(m: &Mutant) -> String {
    let mut words: Vec<&str> = m.target.to_vec();
    words.push(m.filter);
    words.join(" ")
}

/// The place every child cargo runs: the worktree, its own target directory.
struct Bench {
    tree: PathBuf,
    target_dir: PathBuf,
}

impl Bench {
    fn cargo(&self, m: &Mutant, build_only: bool) -> Result<Output, MutateError> {
        let mut cmd = Command::new("cargo");
        cmd.args(["test", "--release"]).args(m.target);
        match build_only {
            true => cmd.arg("--no-run"),
            false => cmd.arg(m.filter),
        };
        cmd.current_dir(&self.tree)
            .env("CARGO_TARGET_DIR", &self.target_dir)
            .output()
            .map_err(|e| MutateError::Cargo { detail: e.to_string() })
    }

    /// Build, then run: a mutant that does not build is ERROR, one whose
    /// suite fails is KILLED, one whose suite passes SURVIVED.
    fn judge(&self, m: &Mutant) -> Result<Verdict, MutateError> {
        let built = self.cargo(m, true)?;
        if !built.status.success() {
            return Ok(Verdict::Error(compile_errors(&built.stderr)));
        }
        let ran = self.cargo(m, false)?;
        let (failing, _) = read_run(&ran);
        match ran.status.success() {
            true => Ok(Verdict::Survived),
            false if failing.is_empty() => Ok(Verdict::Killed(vec![format!("(the run exited non-zero: {})", tail(&ran.stderr, 1))])),
            false => Ok(Verdict::Killed(failing)),
        }
    }

    /// Every distinct suite, on the unmutated tree, builds, passes and runs
    /// at least one test.
    fn baseline(&self, catalogue: &[Mutant]) -> Result<(), MutateError> {
        let mut seen: Vec<String> = Vec::new();
        for m in catalogue {
            let suite = suite_name(m);
            if seen.contains(&suite) {
                continue;
            }
            eprintln!("mutate: baseline `{suite}`");
            let built = self.cargo(m, true)?;
            if !built.status.success() {
                return Err(MutateError::BaselineUnbuilt { suite, detail: compile_errors(&built.stderr) });
            }
            let ran = self.cargo(m, false)?;
            let (failing, passed) = read_run(&ran);
            if !ran.status.success() {
                return Err(MutateError::BaselineRed { suite, failing });
            }
            if passed == 0 {
                return Err(MutateError::BaselineEmpty { suite });
            }
            seen.push(suite);
        }
        Ok(())
    }

    /// One mutant applied, judged, and the file restored before any answer.
    fn apply(&self, m: &Mutant) -> Result<Verdict, MutateError> {
        let path = self.tree.join(m.file);
        let original = match std::fs::read_to_string(&path) {
            Ok(text) => text,
            Err(e) => return Ok(Verdict::Stale(format!("the file does not read: {e}"))),
        };
        if m.before == m.after {
            return Ok(Verdict::Stale("the mutant changes nothing".to_string()));
        }
        match original.matches(m.before).count() {
            1 => {}
            0 => return Ok(Verdict::Stale("the before span is not in the file".to_string())),
            n => return Ok(Verdict::Stale(format!("the before span occurs {n} times, not once"))),
        }
        let mutated = original.replacen(m.before, m.after, 1);
        std::fs::write(&path, mutated).map_err(|e| MutateError::Io { path: path.clone(), detail: e.to_string() })?;
        let verdict = self.judge(m);
        std::fs::write(&path, &original).map_err(|e| MutateError::Io { path: path.clone(), detail: e.to_string() })?;
        verdict
    }
}

/// One report line per mutant.
fn line(m: &Mutant, v: &Verdict) -> String {
    let detail = match v {
        Verdict::Killed(tests) => format!(" — caught by: {}", tests.join(", ")),
        Verdict::Survived => " — NO TEST NOTICED: the law is unguarded".to_string(),
        Verdict::Stale(why) | Verdict::Error(why) => format!(" — {why}"),
    };
    format!("{:<9}{} [{}] ({}){detail}", v.word(), m.law, m.file, suite_name(m))
}

/// The summary, and whether the audit passes: every mutant KILLED.
fn summary(verdicts: &[Verdict]) -> (String, bool) {
    let count = |w: &str| verdicts.iter().filter(|v| v.word() == w).count();
    let text = format!(
        "mutation audit: {} mutants — {} killed, {} survived, {} stale, {} error",
        verdicts.len(),
        count("KILLED"),
        count("SURVIVED"),
        count("STALE"),
        count("ERROR")
    );
    (text, !verdicts.iter().any(Verdict::unproven))
}

/// The repository root the gate binary was built from.
fn repo_root() -> Result<PathBuf, MutateError> {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    if manifest.join("src/server").is_dir() {
        return Ok(manifest);
    }
    manifest.parent().map(Path::to_path_buf).ok_or(MutateError::NoRepo)
}

/// Run the whole catalogue. `Ok(true)` when every mutant was killed.
pub fn run() -> Result<bool, MutateError> {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn out(stdout: &str, code: i32) -> Output {
        use std::os::unix::process::ExitStatusExt;
        Output {
            status: std::process::ExitStatus::from_raw(code << 8),
            stdout: stdout.as_bytes().to_vec(),
            stderr: Vec::new(),
        }
    }

    /// A run's failing tests are named and its passing ones counted.
    #[test]
    fn read_run_names_failures_and_counts_passes() {
        let o = out(
            "running 3 tests\ntest a::b ... ok\ntest a::c ... FAILED\ntest a::d ... ok\n\nfailures:\n    a::c\n",
            101,
        );
        assert_eq!(read_run(&o), (vec!["a::c".to_string()], 2));
    }

    /// The audit passes only where every mutant was killed; a survivor, a
    /// stale anchor or an unbuilt mutant each leave it red.
    #[test]
    fn summary_passes_only_when_every_mutant_killed() {
        let killed = Verdict::Killed(vec!["t".to_string()]);
        assert!(summary(&[killed.clone(), killed.clone()]).1);
        for gap in [Verdict::Survived, Verdict::Stale("moved".into()), Verdict::Error("e".into())] {
            assert!(!summary(&[killed.clone(), gap]).1);
        }
    }

    /// Every catalogued mutant changes its span and names a file and a suite.
    #[test]
    fn catalogue_mutant_changes_span_and_names_suite() {
        for m in CATALOGUE {
            assert_ne!(m.before, m.after, "{}", m.law);
            assert!(!m.before.is_empty() && !m.file.is_empty() && !m.filter.is_empty(), "{}", m.law);
        }
    }

    /// Every `before` span occurs exactly once in its file in this tree.
    #[test]
    fn catalogue_anchor_occurs_once_in_file() {
        let repo = repo_root().expect("the xtask manifest sits in the repository");
        for m in CATALOGUE {
            let text = std::fs::read_to_string(repo.join(m.file)).unwrap_or_default();
            assert_eq!(text.matches(m.before).count(), 1, "{} [{}]", m.law, m.file);
        }
    }
}
