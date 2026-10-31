use super::catalogue::Mutant;
use super::refusal::MutateRefusal;
use super::verdict::MutantVerdict;
use std::path::PathBuf;
use std::process::{Command, Output};

/// The last `n` lines of a process stream, for a refusal's detail.
pub(super) fn tail(bytes: &[u8], n: usize) -> String {
    let text = String::from_utf8_lossy(bytes);
    let lines: Vec<&str> = text.lines().collect();
    lines[lines.len().saturating_sub(n)..].join("\n")
}

/// The compiler's error lines, for an UNBUILT verdict.
fn compile_errors(bytes: &[u8]) -> String {
    let text = String::from_utf8_lossy(bytes);
    let errors: Vec<&str> = text.lines().filter(|l| l.starts_with("error")).take(4).collect();
    match errors.is_empty() {
        true => tail(bytes, 4),
        false => errors.join(" | "),
    }
}

/// What a test run printed: the tests that failed and how many passed.
pub(super) fn read_run(out: &Output) -> (Vec<String>, usize) {
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
pub(super) fn suite_name(m: &Mutant) -> String {
    let mut words: Vec<&str> = m.target.to_vec();
    words.push(m.filter);
    words.join(" ")
}

/// The place every child cargo runs: the worktree, its own target directory.
pub(super) struct Bench {
    pub(super) tree: PathBuf,
    pub(super) target_dir: PathBuf,
}

impl Bench {
    fn cargo(&self, m: &Mutant, build_only: bool) -> Result<Output, MutateRefusal> {
        let mut cmd = Command::new("cargo");
        cmd.args(["test", "--release"]).args(m.target);
        match build_only {
            true => cmd.arg("--no-run"),
            false => cmd.arg(m.filter),
        };
        cmd.current_dir(&self.tree)
            .env("CARGO_TARGET_DIR", &self.target_dir)
            .output()
            .map_err(|e| MutateRefusal::Cargo { detail: e.to_string() })
    }

    /// Build, then run: a mutant that does not build is UNBUILT, one whose
    /// suite fails is KILLED, one whose suite passes SURVIVED.
    fn judge(&self, m: &Mutant) -> Result<MutantVerdict, MutateRefusal> {
        let built = self.cargo(m, true)?;
        if !built.status.success() {
            return Ok(MutantVerdict::Unbuilt(compile_errors(&built.stderr)));
        }
        let ran = self.cargo(m, false)?;
        let (failing, _) = read_run(&ran);
        match ran.status.success() {
            true => Ok(MutantVerdict::Survived),
            false if failing.is_empty() => Ok(MutantVerdict::Killed(vec![format!("(the run exited non-zero: {})", tail(&ran.stderr, 1))])),
            false => Ok(MutantVerdict::Killed(failing)),
        }
    }

    /// Every distinct suite, on the unmutated tree, builds, passes and runs
    /// at least one test.
    pub(super) fn baseline(&self, catalogue: &[Mutant]) -> Result<(), MutateRefusal> {
        let mut seen: Vec<String> = Vec::new();
        for m in catalogue {
            let suite = suite_name(m);
            if seen.contains(&suite) {
                continue;
            }
            eprintln!("mutate: baseline `{suite}`");
            let built = self.cargo(m, true)?;
            if !built.status.success() {
                return Err(MutateRefusal::BaselineUnbuilt { suite, detail: compile_errors(&built.stderr) });
            }
            let ran = self.cargo(m, false)?;
            let (failing, passed) = read_run(&ran);
            if !ran.status.success() {
                return Err(MutateRefusal::BaselineRed { suite, failing });
            }
            if passed == 0 {
                return Err(MutateRefusal::BaselineEmpty { suite });
            }
            seen.push(suite);
        }
        Ok(())
    }

    /// One mutant applied, judged, and the file restored before any answer.
    pub(super) fn apply(&self, m: &Mutant) -> Result<MutantVerdict, MutateRefusal> {
        let path = self.tree.join(m.file);
        let original = match std::fs::read_to_string(&path) {
            Ok(text) => text,
            Err(e) => return Ok(MutantVerdict::Stale(format!("the file does not read: {e}"))),
        };
        if m.before == m.after {
            return Ok(MutantVerdict::Stale("the mutant changes nothing".to_string()));
        }
        match original.matches(m.before).count() {
            1 => {}
            0 => return Ok(MutantVerdict::Stale("the before span is not in the file".to_string())),
            n => return Ok(MutantVerdict::Stale(format!("the before span occurs {n} times, not once"))),
        }
        let mutated = original.replacen(m.before, m.after, 1);
        std::fs::write(&path, mutated).map_err(|e| MutateRefusal::Io { path: path.clone(), detail: e.to_string() })?;
        let verdict = self.judge(m);
        std::fs::write(&path, &original).map_err(|e| MutateRefusal::Io { path: path.clone(), detail: e.to_string() })?;
        verdict
    }
}
